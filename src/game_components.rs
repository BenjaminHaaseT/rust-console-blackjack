use crate::Card;
use std::rc::Rc;

pub trait BlackjackHand {
    fn receive_card(&mut self, card: Rc<Card>);

    fn display_hand(&self);

    fn compute_hand_value(&mut self);

    fn is_blackjack(&self) -> bool;

    fn busted(&self) -> bool;
}

trait BlackjackTable {}
