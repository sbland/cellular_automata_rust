cargo build  --release || exit 1
{
mv './target/release/libcellular_automata.so' './cellular_automata.so' || exit 0
}