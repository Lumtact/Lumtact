export interface LumtractNode {
  id: string;
  label: string;
  depth: number;
}
export interface LumtractEdge {
  source: string;
  target: string;
}
export interface LumtractData {
  nodes: LumtractNode[];
  edges: LumtractEdge[];
  meta: {
    generatedAt: number;
    version: string;
  };
}
