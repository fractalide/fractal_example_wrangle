{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ ntup_list_tuple_tt ntup_list_triple_ttt prim_text ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
