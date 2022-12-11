use crate::read_input;
use anyhow::{anyhow, Context};

pub fn solution() -> anyhow::Result<(usize, usize)> {
    let input = read_input("day03.txt")?;

    let phase1_score = phase1(&input).context("phase1")?;
    let phase2_score = phase2(&input).context("phase2")?;

    Ok((phase1_score, phase2_score))
}

pub fn phase1(lines: &Vec<String>) -> anyhow::Result<usize> {
    let mut accum = 0;
    for (line_index, line) in lines.iter().enumerate() {
        let items = line.as_bytes();
        if items.len() % 2 == 1 {
            return Err(anyhow!(
                "the two bags in line {} do not contain the same many items",
                line_index + 1
            ));
        } else {
            let k = items.len() / 2;
            let bag0 = Inventory::try_from(&items[..k])
                .context(anyhow!("first bag in line {}", line_index + 1))?;
            let bag1 = Inventory::try_from(&items[k..])
                .context(anyhow!("second bag in line {}", line_index + 1))?;
            let common_items = bag0 & bag1;
            let badge_priority = common_items.only_item().ok_or_else(|| {
                anyhow!(
                    "there is not a unique common item in the bags on line {}",
                    line_index + 1
                )
            })?;

            accum += badge_priority.0 as usize;
        }
    }
    Ok(accum)
}

pub fn phase2(lines: &Vec<String>) -> anyhow::Result<usize> {
    let mut accum = 0;

    for (chunk_index, chunk) in lines.chunks(3).enumerate() {
        let line_offset = chunk_index * 3 + 1;
        let a_inv = Inventory::try_from(chunk[0].as_bytes())
            .context(anyhow!("inventory from line {}", line_offset + 0))?;
        let b_inv = Inventory::try_from(chunk[1].as_bytes())
            .context(anyhow!("inventory from line {}", line_offset + 1))?;
        let c_inv = Inventory::try_from(chunk[2].as_bytes())
            .context(anyhow!("inventory from line {}", line_offset + 2))?;
        let common_items = a_inv & b_inv & c_inv;
        let badge_priority = common_items.only_item().ok_or_else(|| {
            anyhow!(
                "there is not a unique common item in the bags on lines {}..{}",
                line_offset,
                line_offset + 3
            )
        })?;
        accum += badge_priority.0 as usize;
    }

    Ok(accum)
}

struct Priority(u8);

impl TryFrom<u8> for Priority {
    type Error = anyhow::Error;

    fn try_from(item: u8) -> anyhow::Result<Priority> {
        if item.is_ascii_lowercase() {
            Ok(Priority(item - ('a' as u8) + 1))
        } else if item.is_ascii_uppercase() {
            Ok(Priority(item - ('A' as u8) + 27))
        } else {
            Err(anyhow!("'{item}' is not a valid item"))
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Inventory(pub u64);

impl Inventory {
    /// Include an item (by its priority) in an inventory
    #[inline]
    fn record_presence_of(&mut self, priority: Priority) {
        let mask = 1u64 << priority.0;
        self.0 |= mask;
    }

    /// If there is only a single item in the inventory, return that item.
    /// Otherwise, return None.
    fn only_item(&self) -> Option<Priority> {
        if self.0.is_power_of_two() {
            Some(Priority(self.0.trailing_zeros() as u8))
        } else {
            None
        }
    }
}

impl TryFrom<&[u8]> for Inventory {
    type Error = anyhow::Error;

    fn try_from(items: &[u8]) -> anyhow::Result<Inventory> {
        let mut inventory = Inventory::default();
        for (pos, item) in items.iter().enumerate() {
            let priority =
                Priority::try_from(*item).context(anyhow!("item '{item}' at position {pos}"))?;
            inventory.record_presence_of(priority);
        }
        Ok(inventory)
    }
}

impl std::ops::BitAnd for Inventory {
    type Output = Inventory;

    fn bitand(self, rhs: Self) -> Self::Output {
        Inventory(self.0 & rhs.0)
    }
}
