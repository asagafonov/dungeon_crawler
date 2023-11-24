pub const MAX_DUNGEON_DEPTH: i8 = 5;

struct MapItemsAmount {
  terrain_pieces: i8,
  monsters: MonstersAmount,
  treasures: i8,
}

struct MonstersAmount {
  weak: i8,
  average: i8,
  strong: i8,
  boss: i8,
}