use crate::card::Card;
use crate::file::CardsReader;
use crate::scheduler::Schedule;
use fnv::FnvHashMap;
use std::collections::VecDeque;

pub struct Qa {
    queued: VecDeque<Card>, // Cards yet to be scheduled
    scheduled: FnvHashMap<u64, Card>,
    schedule: Schedule,
}

/// Question & Answer object consist of these parts:
///
/// * A spaced repetition learning scheduler. User can:
///   * get (and display) current card
///   * assess difficulty of current card which reschedules the card and moves
///     to the next card
/// * FIFO queue of cards not yet schedule (i.e. learned). User can put more
///   cards to scheduler.
impl Qa {
    /// Initialize Question & Answer object from cards iterator. Schedule is
    /// loaded from disk.
    pub fn load(reader: CardsReader) -> Result<Qa, String> {
        let schedule = Schedule::load()?;

        let mut qa = Qa {
            queued: VecDeque::new(),
            scheduled: FnvHashMap::default(),
            schedule,
        };

        for card_result in reader {
            let card: Card = card_result?;

            if qa.schedule.has_item(card.id()) {
                qa.scheduled.insert(card.id(), card);
            } else {
                qa.queued.push_back(card);
            }
        }

        Ok(qa)
    }

    /// Save schedule to disk. Exiting schedule file is rewritten.
    pub fn save(&self) -> Result<(), String> {
        self.schedule.save()
    }

    /// Returns true if all cards scheduled for today has been learned.
    pub fn is_today_schedule_done(&self) -> bool {
        self.schedule.is_done()
    }

    /// Returns true if there is at least one card not yet scheduled.
    pub fn is_all_scheduled(&self) -> bool {
        self.queued.is_empty()
    }

    /// Schedule `count` new cards for learning.
    pub fn schedule_more(&mut self, count: usize) {
        for _i in 0..count {
            let card: Card = match self.queued.pop_front() {
                Some(card) => card,
                None => break,
            };

            self.schedule.add_item(card.id());
            self.scheduled.insert(card.id(), card);
        }
    }

    /// Get "current" card.
    pub fn current_card(&self) -> &Card {
        let item_id = self.schedule.current();
        &self.scheduled[&item_id]
    }

    /// Assess "easiness" of current card and move current the next one.
    /// Easiness spans from 0 to 5.
    pub fn assess_current(&mut self, q: u8) {
        self.schedule.update_current(q);
    }
}
