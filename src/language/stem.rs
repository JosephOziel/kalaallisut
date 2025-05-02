//USE SLICES INSTEAD OF VECS

#[derive(Debug, Clone)]
struct Base {
    stem: Vec<(char, VC)>,
    class: VN
}

#[derive(Clone)]
struct AE {
    stem: Vec<(char, VC)>,
    sandhi: Sandhi,
    class: Class
}

#[derive(Clone, Debug)]
enum Class {
    Affix(VN, VN),
    Ending(VN)
}

#[derive(Clone, Debug)]
enum Sandhi {
    Trunc, // sandhi truncative
    Epen(char), // sandhi epenthetic
    Join, // sandhi joining
    G, // g-sandhi
}

#[derive(Clone, Debug)]
enum VC {
    V, // vowel
    C // consonant
}

#[derive(PartialEq, Clone, Debug)]
enum VN {
    Verb,
    Noun
}

fn check_type(base: &Base, ae: &AE) -> bool {
    match &ae.class {
        Class::Affix(vn, _) => base.class == *vn,
        Class::Ending(vn) => base.class == *vn
    }
}

fn trunc(base: &mut Base, ae: &mut AE) -> () {
    match base.stem.as_slice() {
        [.., (_, VC::C)] => {
            base.stem = base.stem[0..(base.stem.len()-1)].to_vec();
            base.stem.append(&mut ae.stem);
        },
        [.., (_, VC::V)] => {
            base.stem.append(&mut ae.stem);
        },
        [] => {}
    }
}

fn epen(base: &mut Base, ae: &mut AE, c: char) -> () {
    match base.stem.as_slice() {
        [.., (_, VC::C)] => {
            base.stem.append(&mut ae.stem);
        },
        [.., (_, VC::V)] => { 
            base.stem.push((c, VC::C));
            base.stem.append(&mut ae.stem);
        },
        [] => {}
    }
}

fn join(base: &mut Base, ae: &mut AE) -> () {
    base.stem.append(&mut ae.stem)
}

fn g(base: &mut Base, ae: &mut AE) -> () {
    match base.stem.as_slice() {
        [.., ('k', VC::C)] => {
            base.stem = base.stem[0..(base.stem.len()-1)].to_vec();
            base.stem.append(&mut ae.stem);
        },
        [..] => {
            base.stem.append(&mut ae.stem);
        }
    }
}

fn apply_sandhi(base: &mut Base, ae: &mut AE) -> () {
    // if !check_type(&base, &ae) {
    //     return None
    // }
    match ae.sandhi {
        Sandhi::Trunc => trunc(base, ae),
        Sandhi::Epen(c) => epen(base, ae, c),
        Sandhi::Join => join(base, ae),
        Sandhi::G => g(base, ae)
    }
}

//----------------------------------------------------------------
pub fn test() -> () {
    let mut base = Base { stem: vec![('p', VC::C), ('a', VC::V), ('n', VC::C), ('i', VC::V), ('k', VC::C)], class: VN::Noun };
    let mut base1 = Base { stem: vec![('a', VC::V), ('l', VC::C), ('i', VC::V), ('q', VC::C), ('a', VC::V), ('q', VC::C)], class: VN::Noun };
    let mut ae = AE { stem: vec![('g', VC::C), ('a', VC::V)], sandhi: Sandhi::G, class: Class::Ending(VN::Noun) };
    let mut ae1 = AE { stem: vec![('t', VC::V)], sandhi: Sandhi::Trunc, class: Class::Ending(VN::Noun) };
    apply_sandhi(&mut base, &mut ae1);
    println!("{:?}", base);
}