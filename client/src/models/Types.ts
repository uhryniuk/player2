import React from "react";
import { CellState } from "./Cell";

type Board = CellState[][];

type Cell = React.JSX.Element;

type Subscription = {
  key: String, 
  data: Object,
}

export type {
  Board,
  Cell,
  Subscription,
}
