type GameStage = "";
type Phases = "draw" | "action" | "";

export class Game {
  phase: Phases;
  constructor() {
    this.phase = "draw";
  }
}
