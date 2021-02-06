use crate::GraphicsContext;
use turn_based_shooter_shared::battle::{BattleInfo, TilePos};

pub struct BattleClient {
    battle_info: BattleInfo,
}
impl BattleClient {
    pub fn new(battle_info: BattleInfo) -> BattleClient {
        BattleClient { battle_info }
    }
    pub fn draw(&self, ctx: &mut GraphicsContext) {
        let map = self.battle_info.map();
        for y in 0..map.y_size() {
            for x in 0..map.x_size() {
                let pos = TilePos::new(x, y);
                if let Some(tile) = map.get_tile(&pos) {
                    ctx.draw_tile(&pos, tile.appearance());
                }
            }
        }

        ctx.draw_white_dot(self.battle_info.white_player_pos());
        ctx.draw_black_dot(self.battle_info.black_player_pos());
    }
}
