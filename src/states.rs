use bevy::prelude::*;

// pub struct StatefulPlugin;


// impl Plugin for StatefulPlugin {
//     fn build(&self, app: &mut App) {

//     }
// }


#[derive(Clone, Copy)]
pub enum StateRepeat {
    /// State doesn't repeat.
    None,
    /// State repeats whole pattern.
    All,
    /// State repeats from the given index.
    FromIndex(usize)
}
#[derive(Clone, Copy, PartialEq)]
pub enum StateDuration {
    /// Changes the state instantly, the given parameters are instantly applied.
    Instant,
    /// Doesn't guarantee that the whole parameter will apply in given time.
    Fixed(f32),
    /// Stretches the given parameters to happen in the given time.
    /// TODO: Should store initial max time as well to interpolate the parameters!!
    Stretch(f32)
}


#[derive(Event)]
pub struct StatefulEvent<T: State> {
    pub entity_id: Entity,
    pub state: T,
}

pub trait State: Copy {
    fn get_duration(&self) -> StateDuration;
}

#[derive(Component)]
pub struct Stateful<T: State>{
    pub state_current: usize,
    pub state_duration: StateDuration,
    pub states: Vec<T>,
    pub state_repeat: StateRepeat,
}

impl<T: State> Default for Stateful<T> {
    fn default() -> Self {
        Stateful {
            state_current: 0,
            state_duration: StateDuration::Fixed(1.),
            states: Vec::new(),
            state_repeat: StateRepeat::None,
        }
    }

}

impl<T: State> Stateful<T> {
    pub fn from_states(states: Vec<T>, state_repeat: StateRepeat) -> Stateful<T> {
        let first_state = &states[0];
        Stateful {
            state_current: 0,
            state_duration: first_state.get_duration(),
            state_repeat,
            states,
        }
    }
}

//Ideas: 
// Have functions set the state, for example:
//   -  Have the angular rotation be a sinus function
//   -  The speed of a projectile gradually speed up


impl<T: State> Stateful<T> {
    fn increment_state(&mut self) -> Option<StatefulEvent<T>> {
        //TODO: This introduces a one frame delay in instant durations
        if self.state_current + 1 == self.states.len() {
            match self.state_repeat {
                StateRepeat::All => self.state_current = 0,
                StateRepeat::FromIndex(index) => self.state_current = index,
                StateRepeat::None => return None
            }
        } else {
            self.state_current += 1;
        }
        let state = self.states[ self.state_current ];
        self.state_duration = state.get_duration();                

        return Some( StatefulEvent{ 
            entity_id: Entity::PLACEHOLDER,
            state: state
        });
    }


    pub fn update_state(&mut self,time_delta: f32) -> Option<StatefulEvent<T>> {
        match self.state_duration {
            StateDuration::Instant => {
                return self.increment_state()
            },
            StateDuration::Fixed( time_left ) | StateDuration::Stretch( time_left ) => {

                //TODO: Check if the extra frame delay is because of floating point errors, use EPSILON
                if time_left <= 0. {
                    return self.increment_state()
                } else {
                    // Decrement timer
                    match self.state_duration {
                        StateDuration::Fixed( time ) => self.state_duration = StateDuration::Fixed( time - time_delta),
                        StateDuration::Stretch( time ) => self.state_duration = StateDuration::Stretch( time - time_delta),
                        _ => () 
                    }

                    return None
                }
            }
        }
    }
}