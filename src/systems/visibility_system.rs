use rltk::{Algorithm2D, field_of_view, Point};
use specs::prelude::*;

use crate::{Map, Player, Position, Viewshed};

pub struct VisibilitySystem;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>, );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            entities,
            mut viewshed,
            pos,
            player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| map.in_bounds(*p));

                let p = player.get(ent);
                if let Some(_p) = p {
                    for vis in map.visible_tiles.iter_mut() {
                        *vis = false;
                    };

                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}