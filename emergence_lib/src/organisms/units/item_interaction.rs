//! Holding, using and carrying items.

use bevy::prelude::*;

use crate::{
    items::{inventory::Inventory, slot::ItemSlot, ItemCount, ItemId, ItemManifest},
    structures::crafting::{InputInventory, OutputInventory},
};

use super::behavior::{CurrentAction, Goal, UnitAction};

/// The item(s) that a unit is carrying.
#[derive(Component, Debug, Deref, DerefMut)]
pub(crate) struct HeldItem {
    /// The internal representation.
    pub(crate) inventory: Inventory,
}

impl Default for HeldItem {
    fn default() -> Self {
        HeldItem {
            inventory: Inventory::new(1),
        }
    }
}

impl HeldItem {
    /// The item and quantity held, if any.
    pub(crate) fn item_slot(&self) -> Option<&ItemSlot> {
        self.inventory.iter().next()
    }

    /// The type of item that is being held, if any.
    #[allow(dead_code)]
    pub(crate) fn item_id(&self) -> Option<&ItemId> {
        let item_slot = self.item_slot()?;
        Some(item_slot.item_id())
    }

    /// The number of items of a single type being held.
    #[allow(dead_code)]
    pub(crate) fn count(&self) -> usize {
        if let Some(item_slot) = self.item_slot() {
            item_slot.count()
        } else {
            0
        }
    }
}

/// A system which performs the transfer of items between units and structures.
pub(super) fn pickup_and_drop_items(
    mut unit_query: Query<(&CurrentAction, &mut Goal, &mut HeldItem)>,
    mut input_query: Query<&mut InputInventory>,
    mut output_query: Query<&mut OutputInventory>,
    item_manifest: Res<ItemManifest>,
) {
    let item_manifest = &*item_manifest;

    for (current_action, mut current_goal, mut held_item) in unit_query.iter_mut() {
        if current_action.finished() {
            let new_goal: Goal = if let UnitAction::PickUp {
                item_id,
                output_entity,
            } = current_action.action()
            {
                if let Ok(mut output_inventory) = output_query.get_mut(*output_entity) {
                    let item_count = ItemCount::new(item_id.clone(), 1);
                    let _ = output_inventory.transfer_item(
                        &item_count,
                        &mut held_item.inventory,
                        item_manifest,
                    );

                    // If our unit's all loaded, swap to delivering it
                    if held_item.is_full() {
                        Goal::DropOff(item_id.clone())
                    // If we can carry more, try and grab more items
                    } else {
                        Goal::Pickup(item_id.clone())
                    }
                } else {
                    // Something has gone wrong (like the structure was despawned)
                    Goal::Wander
                }
            } else if let UnitAction::DropOff {
                item_id,
                input_entity,
            } = current_action.action()
            {
                if let Ok(mut input_inventory) = input_query.get_mut(*input_entity) {
                    let item_count = ItemCount::new(item_id.clone(), 1);
                    let _ = held_item.transfer_item(
                        &item_count,
                        &mut input_inventory.inventory,
                        item_manifest,
                    );

                    // If our unit's unloaded, swap to wandering to find something else to do
                    if held_item.is_full() {
                        Goal::Wander
                    // If we still have items, keep unloading
                    } else {
                        Goal::DropOff(item_id.clone())
                    }
                } else {
                    // Something has gone wrong (like the structure was despawned)
                    Goal::Wander
                }
            } else {
                // Other actions are not handled in this system
                return;
            };

            *current_goal = new_goal;
        }
    }
}