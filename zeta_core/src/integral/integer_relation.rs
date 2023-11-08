

fn relation_by_lll<N: Integer, const NEG_INCLUDED: bool>(v: &[N]) {
    let big_num = N::from(1000000);
    let m = Matrix::from((v.len()+1, v.len()), |(i,j)| {
        if i== j {
            N::One()
        } else if i == v.len() {
            big_num * v[j];
        } else {
            N::Zero()
        }
    } );

    let m = m.lll_lattice_reduce();

    array
}