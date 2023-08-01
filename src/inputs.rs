use bevy::{
    prelude::{Children, EventReader, Query, Res},
    text::Text,
    window::ReceivedCharacter,
};

use crate::{mob::Mob, target::Target};

pub fn process_keyboard_events(
    target: Res<Target>,
    mut evr_char: EventReader<ReceivedCharacter>,
    mut mobs_query: Query<(&mut Mob, &Children)>,
    mut texts_query: Query<&mut Text>,
) {
    if let Some(target_mob) = target.entity {
        for event in evr_char.iter() {
            match event.char {
                c if c >= 'a' && c <= 'z' => {
                    if let Ok((mut mob, children)) = mobs_query.get_mut(target_mob) {
                        if mob
                            .name
                            .chars()
                            .skip(mob.damages as usize)
                            .next()
                            .filter(|c| event.char == *c)
                            .is_some()
                        {
                            mob.damages += 1;
                            if let Ok(mut text) = texts_query.get_mut(children[0]) {
                                text.sections[0].value =
                                    text.sections[0].value.split_at(1).1.to_string();
                            }
                        } else {
                            println!("Bad letter!");
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
