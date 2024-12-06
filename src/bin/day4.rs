fn main() {
    let test_data = parse(TEST_INPUT);
    let test1_result = solve_part1(test_data.clone());
    assert_eq!(test1_result, 18);

    let part1_result = solve_part1(parse(INPUT));
    println!("Part1 result: {part1_result}");
}

fn solve_part1(data: Vec<Vec<char>>) -> usize {
    let horizontal: usize = RowsIter::from_vec(data.clone())
        .map(|row| count_ocurrences(&row))
        .sum();
    let vertical: usize = ColsIter::from_vec(data.clone())
        .map(|col| count_ocurrences(&col))
        .sum();
    horizontal + vertical
}

fn count_ocurrences(part: &[char]) -> usize {
    part.windows(4)
        .filter(|chars| {
            let string: String = chars.iter().collect();
            string == "XMAS" || string == "SAMX"
        })
        .count()
}
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

struct RowsIter {
    data: Vec<Vec<char>>,
    curr_row: usize,
}

impl RowsIter {
    fn from_vec(data: Vec<Vec<char>>) -> Self {
        RowsIter { data, curr_row: 0 }
    }
}

impl Iterator for RowsIter {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.data.get(self.curr_row).cloned();
        self.curr_row += 1;
        res
    }
}

struct ColsIter {
    data: Vec<Vec<char>>,
    curr_col: usize,
}

impl ColsIter {
    fn from_vec(data: Vec<Vec<char>>) -> Self {
        ColsIter { data, curr_col: 0 }
    }
}

impl Iterator for ColsIter {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let cols = self.data[0].len();
        if self.curr_col >= cols {
            None
        } else {
            let res = Some(self.data.iter().map(|row| row[self.curr_col]).collect());
            self.curr_col += 1;
            res
        }
    }
}

const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

const INPUT: &str = "XMASAXMASXXMMAMAXAXXXMSASXSXSXMAMMAMXMASASXMXMASMSMSSMMSXMASXMSMXMASMMSSMMMSMMAMMSMMMMMSXMAXMSMMMMMSAXAXXXMASAMXXSXMXSSSMSMSXMAMXXXSAXSMSAMX
MSMMMMSMXMXSSSSSSMSMSMSASAAXXASAXXMAXSMSAMMSAMSXAAAAXSAMXMASXMXMASAMXXAAAXMAMMAAMAMXAASAMMMSAMXMAASMSXMSMSMXMASMXMASXMAAXAAAXXSSSMMAMMXAXSXX
AAAAAAAMMSAAMAAAXAAAAAMMMMMXMXMASXMSASXMAMAMASAMSMMMXMXSAMXSXMASXSAMXMSSMMSASXXSSMMSAMMASAAAAXXXSMSAAMXAAMSXSAMASAAMAMAMSMSMSMAAASAAXAMXMMMS
SSSMMSSSSMSSMMMMMSMXMMMAAXXAXAMXMXAMASMSMMMSMMMXASAMXSMSXSAMAXAMAMASAMXAXXMAMXAAMXXMXMMAMMSSSMXXXAMXMAMMSMAXMASXXMASMMAXAAAXAAMMMMSXXSAMXAAA
XMAXMXXMAAAXXXAMXXSMMMSMMSAMXMMMMMXMXMAAMAASAMXSAXXMAXAMXMMSSMSSSSMMASXMMMMAMSSMSMMAAXMASXAAAAASMMMSXSXAXMAMSAMXSSMSXMAMMXMSMSSXSAMXAAXXSMSS
XMASMMASMMMSSSMMSAMXSAAAAAXMAMASAMASXMSMSMXSASXXSMSMSSSSXXAAAAAAXAASAMASXXMMXAAXXAMMASAMXMMMMSMXAAXMAMMMMMXMMASAAAXMASXMSMAMXAAXMASAMXMMAAAM
XAXAAAAXAXAAAAMSMAMAMSSMMSMSMSASASASAXAAXMASAMMAMSXMAAAXMMMSMMMMSSMMXSXMMAMSMMSMSXMMAAAMMXSAAAMSMMSMAMAMAMASXMMMSMMMAMAAAAAMMMSMMAMMSAMXMMMS
MSXSMMXSXMMSSMMMSXMXMAMAAAXXAMAMAMXXAMXSXXAMMAMAMSXMMMMMMMXAXXMAXMASXMMASAMAXXAMXAXMXSAMMASMSXMXMAXXAXAMAMXSAMXXAXXMMMMMMSMSAXMAMSSXSAMXSAMA
AAAMXSASASMAXAAAMXSSMMSMMSSMSMSMSSXMXAMMMMXSSXMSSMAXAAAAAXXMSMXXXXAXAASMSASASMSXSSMMMMXXMASXMXSMMXSXXMMXMMAMAMXSAMXSSXMAAAXSMSMSAAAXSAMASASM
SXXMAMASAMMMSSMMSAXAMXAMMXMAAAAXXAASMXSASXAMAMXMAMAMXSSSXSAXAXMMMMMSSMMASXMXSAAASMSAAMXSMASAMAAASXMXSAMAXMSMSXMASXMASAMMMSXXXAAMMXMAXAMXSAMX
MMMMXMXMXMXXAMXAMXXSMSMXSAMSMSMXMXMMAAMAMMMSAMXMAXMXAMXAASMMMSAAAAAXAXMXMASXMMMSMAXMMSAXMASAMXSXMAAASAMMSAMAAXXAXXXAXXMAXMMXSXXMXSMXSAMAMAMS
XAAMAMAMSAMAAAMSSSMAAXAAMXMAXXXXSMMMMMMSMAXXXMMSSMMSMMMMMMAAAXXMXMXSMMMXXMMXMXMAMMMXAMMXSASAMMMMSMMMXAMXAAMAMMMSMSMMMAXAXAMXXMSMXXAXMASMSAMX
XSMSAXASAASXMMAMAASAMXXMSMSMSXMAXAAMAMAMXMSSXSAAAXAAXAAMXSSMSSXSASXSXAMAMXSXMASXSMSMASAAMAMAMAXAAMAXXAMAMAMAXXXAAMASAMMMSMSMMSAMXMXMSAMXSXSA
MMASMSMSMMAAAASMMMMMXMASAAAAMMMASXMSXMASAMAAASMSSMSSSMMAMXXAXAMXAXAMXSMXSAAASASAAAMSAMMMSASMMSMSMSASMSMMMAMMXXSAMSAMAMAXAMAAMSASXSAAMASAMXXM
MMAMAAAXAMSSMMXASXSXASXMMSMXMAXXAXXAAXASAMMMMMXXAAXXAXAAMXSXMXMMSMSAMXAAAMSMMAMAMMMMXSAMMXMXAAXMXMMSXAAAXMMMAASXMMASASMSSSSSMMAMASMXSXMASMMM
XMSMSMSMSAAXXASXMASMMMAXMAXMSMSXAXMMSMASAMSSXMSMSMMSAMSSMAMAMXMAMMAAAMMMXMAXASMAXAXMMSMSMSXMXMXMAMAMXSMMXSAMMXSAMMMMAMAAAAMAMMSMAMAASAXAMAAM
ASXAAMAAXMSMSAMXMXMAMMSMMASXAAAMSMAAAMMMAMAAAAMAAAXXAXAMMAMAMAMASASMSXAXXXXXSASXSSMMAMXMASMMMXXSMMSMMXXAASXMMAMAMXSMSMMMMMXAMXMMAMMMXAMSSSMS
MMMSMSMXMXAAAAMXMASXMAXAMAMMMMMAAASMSXXASMSSMMXSSMMMSMMMSASASXMASAMMMMXAMXMMXMAXMAXSSSSMAMAASMMXAAMASMMMMSXAMASMMAMAMMXXXXMSSMXSSMMSMSAMAAXA
MAAMAXMAMSAXSAMASMSAMAMAMXXAXMSSMXMAMASMMAMXMXMAAAAAXAAASMSXSXMXMMMASASXXSMAXXMSMMMSAAXMSMSMSAMSMMSXMAXMAXMMXMAASXMAMXXXXXAXAMXAAMASAXSXXMAS
SASMXMASAASAMXXASXMMMMSSMMXSMMAAXSMASAXXMXMXMAMXSXMSXMAXSXSASXSAAAXMMAXMASXSAXXAAXXMMMMXAAAAXMMAAAXAMSSSSSSMAMSXMASMSASMMMMSSMMSMMAMXMMXMXMX
AAXMXMAMXAMMSSSXXXMSSMAMAMAXAMMSAMMAMMMXXASMSASMMXSAMXMASAMAMASMSMSSMXMSXSMXAXMSMMXXAAASMSMSMSSSSMSSMXMAXAAAXMXASASASMAAAMXAXAAXXMAXAXMASAMX
XXAAAMXSXSXAAAAMMMXAAMXSAMMSAMXMAXMASMAXSASASASAMXMAMXSASMMMMAMXMASAAMAMAMXMXMAMASXSMSMMAMAAXXMAXAAMAXMXMSMMXSXXMASAMXSXMMMMSMMXMSSSSXSXXAXX
MMMSMSXSAXMMMSMAAMMMSMAMXMASAMXSAMXMXMAMMXMXMAMXMASAMXAAMXMAMAMXMMMMSXMMAMMSAXXSASAAXMXMAMMMXSMMMMMXSMXMAMAMASAMMAMAMMXXMXAMAMAXMAXXAASMSSMM
ASMMAMAMXMAAMXXSMMXMAMMSSXASAMXMXMAXMMXSMMMAMXMASXSMSAMXSXSASAXXXAAXMAXSAMASASAMXMMMMSASMSASMXASXXSAAMXXMXAMAMXXMAMXMAXMAMMXASXSMMSMSMSAAXAM
MAASAMSMSSSMSXXXMAAMASMAMMAMXMAMAMMSAMXMAXSASMSAMMMXMXXXAASASAMSSSMSMSMSAMAMSMXSAMXMMSAMASASXMSMAAMSMMMMMSAMXSSSSMSMMMSSMXSXMSMXAXXAMAMMMSSM
XMXMAMXAAAXMAMXXMXMSAXMAMXMASMSSSSXSXMASMMMXMAAAAASASMSMMXMMMMMAAAASAMMSMMMSXXAXASMSAMAMMMXMASAMMMMMXMMSAAXXASAAMAAXAAAAMASAMXXMXMXSMAMMXAXM
MXXSXMMMMMMMASMASAMMXMSMSXAXXAXAAXAMAXASASMMMSSMXASASAAMXXAXAAXMMMMMAMASXXXAMMMSMMAMMSAMAMASMMMXAAAXXXAAAMSMMSMMMSMSMMSXMASAMASXMXAAXAXXMASM
SSMMAMXMAMAMAXXAMXSXMAAASMMMMMMMMMMSSMMSAMAAAAAMSMMMMXMSSSMMMXSAMXXXAMAXMAMXMSAAAMMMASASXSASXXASMSSSXMMSMMXAAXAXAAXAXMMXMXSAMAMAMMMSSSSXSAMX
AMAMMXSXMASXMSMXSXMXAMMMMAAAAMMMMXXAAAASMMSMMSSMAMXSXMXSAAXXXAMXMASMXMASXMMAMMSMSMSMMSAMXMAXXAXMAXMAAXXMXMXSXSMMSMSSSXSASXSXMAXAMAXAAMSAMXSX
MSAMSMSASAMAMMMAAAMAXXAAMSMSXXAAAMMSSMMMAAXMXXAXMSAMASMMSMMMMMSXMAMAAMMMAAMMSAAXAMXAXMXMSMAMSMMMMMMXMMMMMSAMMSAAAAAXMAMMMAMASXSSSSSMSXSXSASX
XMAMSAMXMASXMASASMMAXSSXMAAAMSSMXAMAMXSXMMSSSSMMXMASAMAAXXAXAAMAMASMXSASXMMXMAXSMXSAMXSMMMXMAAAXSAXAMXAAAMAXAXMMMMMSSSXSMAMAMMXAAXXXAAMXMASX
MSSMMMMXSXMASASAMASXXMAMSMMMMAXSMSAMXASAMAXXAAXAXSMMMSMMMSSSSMSAMASAASASAMXMXMXXXXSASASAAXAMSMMXSMSMXXXMXXXMMXAMXMAXAAAMSSSXSXMMMMMMMSMXMXMX
MAMASAMXMASAMAXMMAMXAMSMMXXSMMSMXMAXMMSAMXSMXMMMXXAAASXMMXMAMAMMMAMMMMAMMMAMAMMMMMSAMASMMSSMXASMXMAXXMASMXMXSAMXSMSMMMSMAMMMMASAAAAAXAMAMAMX
MMSAMAMAMMMSMAMSMSSSXXMASXMAAXMASXSMSXSMMXXMXAAXXXMMXMASMMMSMAMXXAXAMXMAMMXMASAAAAMXMAMAXMXAMAXMASASXSMXAASASMSXMAXSXXXMASAMSAMXSSSXSASMSASX
SAMMSASMXXAXMXSXAXAXMASXMAMXAMXAMXMASAMMXMAXSSMSSMSMSSSMSAAAMAMXXXSSSXMASMMSASAMMSSXSAMMMMMSMMMSXSAXXAAMSMMAMXMAMXMXSXSSMMAMMMMXMAAASAAASAMX
XAMXSASASMAXXMMMMMMMAXMAMXMASMMXSXMMMXMAAMSAMXXMASAAAMMASMSMSMSMMMMAAXMASAXAMXAXSXMAXMXXMAAXAAXMAMXMSMMMMAMMMSSMMSSMXMAAMMSMSASAMMMMMMMMMAMX
SAMAMMMAMXMMSXMAAAXMMXMSMSXMXAMXXMXXXMASXSAAMASAMXMMMSMMMXAXAAAMAAXMMXMMSMMXMMMMXAMSMMMSSMSSSMSSMXAXXAMMSAMXXAAAAAXAAASXMAAASASXSAMXXXSMMXMX
SXMMASMSMAMAXASMSSSSXMMAAXAMXAXASASAXMXXMAXMMASMXSXXASAMSMMSMMMSMMSXAXSXSXMSMSSSSXMAAAAMAAAAAMAAXSAMMXSASXSAMSSMMMMMXXXXMMMXMMMASMMMXMXMASMM
SAMXXMAAXXMAMMMAAAAAASXSMMAMXMMMMXMMMMAASASXSXXMASMMMMAMSAMXAXMMMMMMMMSAMXXSAAXAAASXSMSMMMMSMMSSMMXMAXMASAMMAXXXXSASMXSXSAMAMAMAMAASASAMXSAA
SMMSXMMMXSXSXXMMMMMMMMAAMSSMAXAXSASASMAXSAMAAMXMAXAASXMMSXMMMMAAAMAMXAMAMMXMMMSMSMMXMXMXSXXXXXXAAXXMSSMAMMMAXXMMMMASAASASASXSXMXSSMSASAMAMMM
MAAMMMAXMSAMSXXMAMSSSMMMMAASXMMMSASASMMMMAMXMXSMMMMMSAAASMSXSAMSMSXSMMSMAAAMAMXAAAXXSAMXMAMXMMSSMMXMMAMMSSMSMXAAMMMMMMMAXMMAMMMAXMASAMXXSSSS
SMSSMSASXMAMAMXSASXAXAMXMSXSMXSAMXMMMASXSXMMXMSASAMASXMMMAXAXSXMXMAXMAAMMSXSASMSMSMMMMMAMMAAAAMAAXMXSMMXAMXAASMMSXXAXMMSMXMAMAMASMMMMSXMMAAA
MXMAASXSXSAMMSASXSMSMMXSXMASAASXSMSXMASAMMSAAAMAMAMASXMSMSMSMMMXAMMMMSMSXMAMXXAAAAXXAXSASAMSSSSSSMSAMXSMXSMMMMAAMMSMSMAAAXXXMMXMXAAAASMMMMMM
AASMMMXMASXSXMMSAMXMXSAMASAMMMSASAMXSXMMMASXMMXASXMAMAXMAXAAMXAXSASMAMASAXMSASMMMSASAXMASMXMAAAAXAMASAMMAMAXSXMAMXMAAMXSSSSMMSAMMSMMMSASXSAM
SMSASMAMMMASAMAMAXAMMMASMMASXXMAMXMASAMXMMXXMAMMMMMSSXMMMMSMSMSMXAMAXMASAMMMMSXAAXAMXSMMMMAMMMMMMXSMMASMAMXXMAXMXSMSMSXXAAXAASAMAXXMXSMMAXMM
MASAMMAXAXMSMMAMSSMMASMMXSXMMXMXMSMXSMMMMMMSMSXXAMAMAXMAMMAAAAMMMAMSXMXMXSXAMSMMMASMASMMASXMAXAXAXMAMXMMSSMXXMASAMMMAAXMMMMMMXXMAXAAAMXMMMSA
MAMAMSMSMSAMXMASAAASXSAMASASXXSAAXMMMAAAAMAMAMXSXMASMMSSSSMXMSMMXAMMAMXXMAMASXAXAAMMXSASXMAXSSMMXMAMSASAMXMSAMXMASAMMMMAAMSMSMMMSSSMSSSXAAMX
MMSXMAXAAAMAASXMMSMMAXAMASMMAXSMXASASXMSMXAMMMMAASASXXXAAMAASMMXSASMMMASMAXMMMMMMMSSMSAMXMMMMAAAXSAMSAMAMAMMASAAAMASMAASMSAAAAAAXXMAXXMXMXMM
SAXMSMSMSMXSXSAMXXAMXSSMXMAMAMXMSASMSAAAASXSMAXMMMAXXMMMMMSMSAAXSMMAXMXAMXSXMASASAMAMMAMXASMSSMMXMAMMXMAMMXSAMXMMSXMMMXMXSMSMMMMSAMMMAAMSMSA
MXMXAAXXAXXAMXASAMAMXXXAAMAXAAAAMAMMMMMMXMAAXXSMSASMMSAXXMXASMMXSASMSMSMXXMXSXSAMXMAMSMMSXAAAXMSSSMMMSSXASAMXSXMASMMXXMMAMXXMSSXSXMASXXXAAMX
XMSSMSMSMSXXASMMMSSSMMSSMXMMSMMMXXSMSMASXMSMMMMXAAAAASAMXMMXMXMASAMAAAAMMAMXMMMXMMSMMSAMXXMMMMSAMAAXXAASMMAAAXAMASAMXXXMMMSXAXMAXMSXSXMMMSMS
MMAMAMMXMXMXMMXXXAAAAXMXMASMMXSXMAXAAMAMMMMASAMAMXMMMSSXAXMSMSMAXAMSMSMSMXMAXAAAMAAMXSAMMAXSAMXMSSMMMAXMXSXMSSMMAMMMASXSXAXMMMMXMXMXMMSAMAAA
AMASAMMAMAMXMMAMMSMSAMXSSMXAXASAMAMSMMASAAMSXSMSSMXMAXMSSSXAAAMSSMMAMAAXMAMMSMSSSSXMAMAMAAMSAMMXMAMXSSXAXMXXMAAAAXSMMAAMMMMAASMMSXMASAMASMSM
MSXSASXXSASAXMASXXAMXSAMAMSMMASXMXXXAXMSMSXXAXSAMXAMSXMXMSXMSMSMAASXMMXMMASMAAAAAXAMSSMMSAMXAMAAXXMASMXMSMMMSAMXMMXAMMAMAAXSMSXAMXSSSMSMMXMX
ASMSMMAXMASMXMXXAMXMAMXXAMAXMXMAMMMSSMMXAMMMMMMXSSMXXASMMSXMXMAMMMMAMSXXMAXXSMMMMMXMAAXAMXMSXMSMSSMXSAAASAAXSAMXSXSXMXXMXXMAMXMSMASAMXAAXAMA
MMASAMXXMAMXMSSSMAAMXSMSSSXSXSSSMAAAAAXMAMMAAAMAMMMAMMMASXAXMSASAMXSASXXMSSMMAXXAXSMSSMMSMXAMXMAAXMASMMASMMXMAMSAAMXMAMSASXMMAAAMSMMMSSSMMSS
XMSMXMMAMASXMAAAXSXSASXAMXAMMMAMXMMSSMMMMMASMMSSMMMSSSSSMMXXASAXXSAMMMAXMXAASMMSSSSXXXMSAXAMMAMMMSMMMXMXMAMASMMAMMMAMAXMASAXSXXXSMASXXAAAXAM
MSMMSAMXSASAMMXMMXAXAXAMXMAMAMAMXSXXAMXAMXAMAMAXAAAMAMXAMSASMMAMXMXSAMXMXSAMXXAMMAMSMXMSASMSMMMSAMAMSXSXSMMAMMXMAXXXMXMMMMXMMAMSXMAMXASMMMMS
SXSASXSAMASMMSAXXMMMSMMXAAMXSMMSAMAXMSSMMMMMXMMMXMXMAMMSMSAMXXASXAASAMMMXXAMSMXMMXMASAASAMAAMSAMXSAMXXSASAMAAMMXSASMMSASAAAMMAMAAMXXXAMXMASX
SAMXMAMMXXXXAMXXXMAAXAXXMMSAMAAMMSMMXXXXAXSASMSSSSXSASXXAMAMXSAXMMMSXMAXAMXMXAMXMAXXMXMMAMXMSMXMAMMSMAMAMXMASXSAMMMAAMAMXMMSAASMAMASMMSMMSSX
MAMSMSMASXSMSSSSMSMSSSMSMAMASXMXMAMASAMSMMAASAAAAMXSAXAMXMMAXMMMXAAXASMSSXAMMAAMXSSMMSXSAMMSXXSMMSAAMMMSMSAAMAMXSXMMSSMSSMAXMMSXMMASMXAMMAMX
SSMSAXMXSAAAASAAXMAMMAAAMAMAMAAXSASMMAXAXXMMMMMMMMMMMMSMMXXMXAMMXMSSXMAAMSMMAMSAAXAMXAAMXXXMAAXMSXXMXAAMAMMMMAMAMXMAMAMAAMSMSXSASMASASASMASX
XMASMMSXMMMMMMMMMMMMSMMMSXSXSMMMSXMXSXMASMXMXXMXMXMASAMXAMSMSAMXXSAXAMMMMAMSXXAMXSAMXMSMMMSMMMXXXXMSSMMMAMASXMSASAMASXMXXMXAMASAMMAXMMXMASXX
AMMMMAXAMXMSXSXMXXMASXSAMXSXAMAAXMMXAXMAMAAAMXXXMASMMAMSSXMAMXMXXAAMMMAXSAXAMXSAMSAMXAAAAAAXAXMXXAXAAAMSMMAMAASAXXSXSXSMSMMAMXMAXMAMXSASMXAS
XXSAMMSMMAMMAMMMSXMMXAMXXAXMMSXSAMXSSXMAXSXMASMAMAMMMMMAMMSSMMSSSMSMMSMXSMSMMMMAMMAAXSSSMSSXSSMAXSMSXMMSXMASMMMSMMMAMAMAAMXAMXMASAXXASASAMAS
SXSAMASASASMAMXAAXSXSSMSSSMSXAMXMSAAXMAXMAMMAAMAMXSMSMMASAAAAXAAMAAMXAXXXAXXAASAMXSAMXAXMMMMXAMXSXAXXSXSAMXMAXAMXAMAMAMSMXMAMAMMSASMAMAMMMAM
AASXMXSASAMMAXMMXXXAXXAAAMAMXSAAMXAMMSAMMSMMXSSXSAAMAMSMMMSSMMMSMMMSSXSMMMMSSXSMSAMMSAMXSXMMSAMXXMXMXMAMAMSXSMMSSXSXMXXXASXXSASXMAMMAMXMAMAS
MMMASXMMMMMMSSMMSSMMMMMMSMMMAXSMSAXXAMASAAXXAXMASMMMMAASAAXMXXMAMXSXXAAXAAAMXAMXSAMASAXMSAMXSXMSMXAMMMAMAMXAMXMAXMAMXSAMXMAMAMXMMAMMXMXXASAX
XASMMAAAAAAAXAMAAAXMAAXAXMSMSMMMMMSMXSMMMSXMXSMXMAMSXSASMSXSMSSSSMXAMSMSSSMMMXMASAMASASXSAMAMAMMMSMMAXAMSAMXMAMAMXASXSXSAMMXAXXMXAMMASMMXMMS
SMSXSSMMSMMMSAMMXSMMSSMMXMAAMXAAAXAXXMASXMXSXMMAMAMXAMASAXMMAXXMXMMXMXXXAMAAASMXMAMMSAMASXMAMSASAMAAMXMSMXAXMSMSXSASXXAMAMXMMSMMSMMSASMMAAAS
MXSAMXAAMAMAXXMSSMAAAAAXMSMSMSSMSXAAMXXSAMAMMAAASAMMMMAMAMXMSMXAAXXXMMASASXMMXMXMAMXMAMXMASASXMMASMXSAMAMSSMMAAXMMASXSMSSMMAXMAMXXAMMSASXSMS
XAMAMMMMSXMAMSMXASMMSXMMXAAAXXAAMMMMSSMMAMXMSXAXXMXAXMXMXMASAAMSXSMAAAAMAXMSXMMASASASMMXSXMXSAXMXMAASMXXXAAASMMMXMAMMMAAXAMSXSSMXSMSASAMXAXX
MSSSMSAMXMMSAMXSMMSMXXSXSMSMMSMMMAMXMAASAMAMAMXXAMSSMMSMMSXSMSAAAAXSMMXMXMAMAAXMXXSASXMASMMXSAMXSMMMSXMSMSSMMSSMXMASAMMMMMMMXMAMAMAMAMMAMMMM
XAAAAAAMAXMASMAMMASXMXMASAXMAAAMSMSASMMMASXSMAMMSMAAAAXAXSXSXXMMXSAMXSMSMSMSSMSMSXMAMAMMSAMASAMXAAXXMAXAAAMAAXMXMMMSXSAAASMSASMMASAMXMMAMXAX
MMSMMMSMMSSXMMASMMSAMMMMMMMMXSMMAASASXMSAMAMXSAAXMSSMMXXMSASMSXXMMASAMAAAAXXAMAAAAMAXXSASMMMSAMSSMMSMSAMMMSMMXSAMAASMSMSMSASXMMSMSASMMSMSSXS
AAAASXAAAAAXXMMMXASXMAMAAAMXMXXSXXMAMAXMMSAMMXMXSXAXMMXSAMAMAMMSMSAMXSSMSMSXMSSSMXSXXXMXXXXXSXMMAAXAAXXXAXAAXASMSMMSAMAXAMXMASXAASAMMMAAMMMX
MXXMMXSMMMSSSSSSSMXMSASMSSMSASMMSAMXMSMAAMAMAXMASMSXSAAMXMAMXMASAMAMXAMXAAXXXAAXXMMXXSAXSASXMMSSSMMMXMSSSSSSMMSAAMSMMMAMXMASXMSMMMXMASMSMSSX
SXSSSXASXMAMAASAMXAXSASAXAAMXMAXXXMXAXSMMXMMSXMMXAMAMMXSASXSXMASMSAMXMMSASXMMMSMMAAXMASMAMXAAAAAAMXMXSAXMAMXAXMMMSAMXMMSSSXMAXMXSMXSASAAAAMM
MAAAMAMAXMAMMMMSMMMMMXMMMMMMASXMXSSMXMAXSAMMXASXMAMXMAASAMASAMXXXMXSAAXAMXASAAXAXMXMAMMXXXSSMMSSXMAAXMAMMXMSMMMAXSMSAXAAXXMXSMSAMXASAMXMMMSM
MMMAMAXMXSSSSMSAMXXSAMXMASXSXSASAAXAXAXASXSAXAMAMMMSSMXSAMSMMSXMXXAMMSMAAMXMASMSMMXASXMMSMMAXMXMASMSMSSMMAMXASXMMSXSMSMMSMSAMAMASMXSXSXXMASX
XSSSSSSXMAAAAXSASMXMASXSASMMMSAMMSMSMSSXMAMXSSSXMAAXXMASXMXAAAASXMXSXMXMMMSAMXAAAXSMMAXAMXXSMSSMXMAXAAMAXASXMMAMSXAXAXAMMAMAMAMXSMXMAMAMXAMX
AXXAAAXXAMMMMMSXMMMSMSAMXSAXAMXMAMAXAXSAMXMAAXAASMMSAMXSASMMMSSMAXAAMSAMXMMASMMXSAAASAMXSSXMASAXAMXMMMSXSMSAXSAMAMSMSSMMMMMSMMMAXXAMAMSSSMXS
SXSMMMMSMXAAXMXMMAAAMSMASXMMXSAXXMXMSMSXMASMMMSMMAAXXMAXMMAMAMAMSMXSMSXXAXMAMAAAXMSAMXXAAXAMSXMASMSSMAMXMAMMMSMSMAAAAXXAXXAAMXMAMXXSMSAAAMAM
XAXXXAAAMSXSMSASMMSXXXXMXMASXSASMXMAXAXAXXXMXAMMXMMXMSASASAMASMSXAMXAMASXMMASMMMMXXMMAMSMXXMMAMXXXAAMXMAMSMSASXAXSMSMMXSSMSXMAMSSMASMMMSMMMS
XSMMMMMSXSAAASASAMMXMXMMAXSAMXXMMMASMSMXMSXMMSSMSXSAXXAAMSASXSMSMSMMAMMMSAMAXAAXXAMAMSMAASXXMASXMMSSMSMXMXAMAMSXXAXMAXAMAAMSSXSAAMAMAAAAAAAM
AMASAMMXAMMMMMASAMXASAMSSSXXMMASASMMAXXXMXMAAAXAAASXSMXMASXMXSXXAMXMAMXAXMMXSSMMMXMAMAMMXMAAXSMMSAAAAXXMASMMSMAMXMXSAMXSAMAAMMSMSMXMMMSSSMSS
XSAMASXMMMXAAMMSMMSXMAXAMMAMXSMSASASMSMSMAAMMSMMMXMAXAMXASMMAMXMSMMSMXMSMXMXAXXMASMMSMSSSMSSMAAMMMSSMMMMAMXAMMXSAXXXMAMXMMMASAMXMMXSAAXXXMAX
XMASMMAAMXSSMXAXAAXMSMMMSSSMMSAMAMAMAXAAXXMXSAAAXAMXMAXMMSAXAMAAMXMASAAXASAMXMXMASAXSAMXAMAMMMMMAAXXASXAMXMSSXAXMSXSXSASXSSXMAMAAAAMSMSSMSMM
SXXMAXSSSXAAAMSMMASXAMAXXMAAAMXMAMXMAMSSMSSXMASXMSAAXMASMSXMASMSSXSASMSMAMMMAMXMAXMMSAXSXMASAXMXMSXSAMXXMXXAAMSSXXAMAAXAAXXMXSMMMMXXXMAXASAS
SXSSSMXAMMMMMMMAMAAMMSMSMSSMMSSSSSSMMXMAAAAAXMAMAMMSMMMMAXMSAMXAAAMAMMXMAMAMMSAMAMSASXMMSSXSMSSXXAXXMAMAMXMMSAXMXMAMASMMSMMMAXAMXXMAMAAMMMAM
SAMAMAMAMXSXAAXAMXSXXMMMAAXAMXAAAAXXMAMMMMSSSMSMXMAAAAAMMMXMASMMMMMXMSXSSSSMASASASMASXMAAMMSXAMMMSSSSMMMAMAXMXXMAMXMMXAXXAXXMSAMAMMAXXMSXMAM
MAMAMXXASMSMMMSSMMXMXSAMSMSSMMMMMMMSSMSXMXAAXXMMMMSSSSXSXMMSAMAXAMXMMSAMXAAMXMAMMSMAMAMXMSAMMMXAAMAMAAAMSMSMSMSMSMMAMSMMSMMMXSXMASMSMAAAXSAS
MAMAXMMMSMMASMAMAXXAMXMMXMAXASXSXAAMAAAAMMMSMMAXAXAMXAXSMMAMASMMSSMAAMAMMSMMAMMMAXMASMMSXMASMMSMXSASMMMXMAAAAAAAAAAMXAMXAXXMAMXMXAAAMXMMMXAS
SSSSMSAXMASXMMASMMMSMAMAXSMMXMAMMMSMMMSSMAMAMMMASMSSMXMMAMMSXMAXAAMSMSMMAXASXMAMSSSMSAAMAXSMMXAAAMXSAMXAMXMSMSMSSSMMSSSSMXSAMXXAMMSMMSSMMMXM
AAAXASASMXMAXMXMXAMAMAXAAAXMSSMMXSAXAAXAMAMAMAXXXMAXMXXXAMXAXSXMAMMAAAAMASAMMSSSMAXAXMMXSMMAMSMMMSMSMMMXSXMAAXXXMMAXAAXMAAMMAMSXXXAAXAAASXSA
MMMMXMASMMSMMSXMSSSSSSSMXSAMAAAAXSASMSSMSSSSSXSAMMSSMMSSMSXMMMSMSASXSMSMASAXXAMAMMMSMXMAXAMXMAAAXAXMAMSAXASMSMXAMMMMMSMXSMSMAXAMXSMXMXSMMAAS
XMXSAMASAAXAXSAMXMAMAXXMMMAMSSMXMSXMXXXAAAXMAMMAMAMAAAAAAAXXAAAXMASAMAXMMSAMMASXMXAAAAXAMAMMSMSMMSSXMAMMSMMXMASMSAXAAAXMAMXMSSSMMXMASXMXMAMX
AMASXMASMMSXMMAMMXMMMMSXMXMMXAMXXXAMXSMMMSMMMMMAMXSSMMSSMMMMMMMXMMMAMSXSMMMMSMMMSMMXXMMXMAMXSAAAMASXMAAXAAAMMAMASMSMXXXAMMMAMXMAMSMASAXMASXM
XMASAMASAMXMMSSMAAXAAAMAMASXMSXSSMMMAXXMAMAXAMMMSAMXXAMXAAXXSASXAMMXMXAMXAXXMAAAAMXSXSASMMSAMMMSAASXSMSSMMSMMASAMXXAASMMXSXMXMMAMXMSSXMAAMAA
SMMSXMXSXMAXXAXXMASXSSSMMAMAAMAMXAAMMSSMSSXSXSAAMXSMMSSSSMXASAMSAMSMMMAMSMMMSSMMSSXAAASMAMMMMSAXMXMAAXXAAAMASXSXSXMMMSAMASAXAMSXSAMXMMAMASAM
MAMSMSMSASXSMMSSXMAMXAXASXSMMMMMMXMSXAXAMXASASMMSMSXAMAMXSSXMAMMMXXAMXAMASAXAMXAMAMMSMMSXMMXAMAXMXMMMASMMMSAMASAMASAMXMMASAMXMAXXMSAXSAMXXXX
SAMXAAASAMXXAAAXSAMXMMMMAAAXSXMASXMMMSXSAMXMAXMASASMSMAMAXMXXMXXMASXMSXSXSXSASMXSAMXXAMXMSMMXXMMSAAAMXMASXMAMXSASAMAXMXMASXMAMMSMMMAXAXMXSMS
SASMSMXMMMMSMMMSAMXAMAAMMSMMMAMAMAAXAXSXXXXMXMSAMAMAMXMMSSMAXXXSMASAAAXAXMASAMAXMXMXXAMASAMXSAAAXXSMSASXMASXXMSAMMSMMSAXXSASXSAAXAMSMMMMXXAA
SAMMXSSMMAAAMSMMASXMMSSSXAAAXAMMSSMMSSMMSAMMSAMXSSMAMAAAXAMASAAAMAXMMMMMXSAMAMSMMAMMMXSMSXMASMMMSMMMSMXASMMMMAMXMMAXXSMSXSMMAMMSMMSAAXSXSMSM
SAMAAXMASMMMMXAMAXXSAAMAMSSMSSXXAMXMXMAAMASAMXSMMAMAMXMMXMMASMXMMMSMMASAMMASAMXAMASASAAMMMMAMAAAMAAAXMSMMAASMMMMMMASXSAXMXAXAXAXAMXMSMMAXAAX
MAXMAMXXMMASASMMSMAMMSMSAMAMAMMMSMMXAXMMSAMXSAXAXXXSSMSXSXMMXXMXAMAAMAXXASAMXMXSMMXAXSSMXAMAMSMMMSMSMXAMMXMAAXSASMAMAMAMASMSMSMSAMXXAXMAMSMS
SXMMSSMSASAMAXXAAMAMMXAMXSAMAMMAMAMSSMMAMXSAMMSSMSMAAAXAAMXSXMASXSSSMSSMMMXXASAXASMSMMMXSXMAXXASAMMAMSAXAMXMSMMAMMAMXMAXXXXAAAXSAMXSMSSSXMAX
SASAAMASXMASAXMSMSXSXMXMASXMASMASAMSASMASAMXSXMXAAMMMMMMMSAMAMAMXMXAMAAXSXMXXMASAMAAXXSASMMSXSAMMXMAXXAMSXSAXXMXMSXSASMSSMSMMMMMXMAAAAAMMMSM
SAMMSMMMAAXMMSXMAMMMXMSMMMASASMAMAMMAMSAMMSAMXMMSMMXMAMMAMXSAMASXXXMMXMMSASMSMXMAMSMSXMASXAXMMSMXMSXSMMMAAMAMXMAXAAMMMAAAAMSXSASMSSMMMXMAAAA
MAMXAAXMXMMSSMAMMMAXAMMAXXAMXMMXMAMMAMMMSAMMSAXXMASAMAMMASASMSAMMMMXMMSAMAMAAXMSAMAMAMMAMMMSAXASAMXAXXSMMMMAAASXXMAMAMMMMSMMXMASAAXXXASMMSSS
SMMSSSMMSAMXAMMMSSSMMSSMMMSSSMSMSSMMAMAMMXMXMXSSXMAXSASXSSXSMXMMASAAXAAXMAMSMMASXSMSAXMAXAXXMMMSXSMMSMMASXMXSMAXAMXSASMMAMAAXMAMMMMMMMASXXAX
SAMXMAAXAASXMMSAAXXMAAAAXXAAAAMAAAXSSSSSXMSSMMMXSXAXSMMXAMXXMAXSAMSSMAXSSXXAAMMMAMMMASXSSSMSXMXSAXXXAMSAMASMXXAMMSAMXMASASMMMMASAXMXAXMAXMAM
SXMMSSMSMSMXXSMMSMSAMSSSMMMSMMMMSSMAAAAAXAAAMAXAMMSMSSMMMMMXMAMMAMAMXSMAAMSXMMAMXSXMMMAASAMSASAMXMMSMXMASAMASMMSAAAXAMAMXMMASXMMXMSSMSXMMSAM
XMAXAXMAMAAXMSAMAAAMMAAAAXAMAXXXMAMMMMXMSMSSMSMASAMMMAXMASAAMSMSAMXSAMMXMXSXSSXSAMMXSAMXMMMMAMAXMMXAMXXMMMSSXASMXMASMSMSMXSASASMMAXAXMASASAS
SASMMSSMSMSMASAMMSMXMMSMMMSSMMMASMMXXSMMMMAMAMSMMMMASAMXSMMSMMASXSAMXSAAXAMAMMAMMSAAMAXSMXSMAMAASMSMSSXMAXSMMMMMMXAXXAAAAMMAXAMXASXSMSMMMSAM
ASXXXAAXXXAAMSXMAMMXMAAAXXAAAXMAMAXMXMASAMXMMMMXAXSXMSMSXXAMASAMXMAMMSMSSSMAMXMMAAMMSAMXSAASAMSSMAAXAAASMSXXAAAAXMASXMSMSMMSMSMMSXMAXAMXAMAM
MMSSMSSMMSMSXXMAXMXAMXSSMMSSMMMSSMXMAMAMAXAMAAXMXMMXAAXMASAMMMXMXSXMAXAMAXXXSMXMXMAAXXSXMXMSMXMAMSMMSSMMXAMSMSMSSSXXAAAMAXMAMXAXMAXXSXXMSMAX
XAAXXXXAAXMAMMASMMMMMXAMXAMAMXAAXXXSAXAXMSXSSMSAMMASXMSSMAXMXMXMAMMMSSMMMMMMXMAXAXMAXMXXMSMMMMSAMXXAAAAXMAMSAMAMAMXMMMMSSMSMSXSMMAXMMMXMASMS
XMMSMMSSMMAXMMAAAAMASXMAMMXAMMSMMMMSMSSSXSXAAXMASMAMAAXMSMAAAMMMASXXAXXAAAAMASXSSSMMXMAAAAXAAMXXXXMMSSMMMSAMXMXMAMSAMXAXMAAXMXXAMMSAAAMSMSXS
MAXAAAAMASXASMMSSMMAMASXAMSSMAMAMMAMXAAMASMMMSSMMMAMMXMAXXMMMXMXMXXMSSSSSXMSAMMXMAXAAMSSSMSMSSXSSMSAXAXXXMASMMASXSXXMAMMMSMAXASAMAMMXMXMXMAM
MXSMSMXSAMXXMXMAMMMASXMMSXAMMMSAAASXMMSMAMAMAXAMXSASXMSSMXSASAMASXMAMAMXAXXMMXAMSSMXSAAXAXMMAMXAAASMSMMMXMXMASXSMSAMMAXSAMXXMASAMSXMXSMSMMAM
SMXAMXAMAXSXMAMXMASMMAXAAMMSMXMMMMMAMAMMASMMMSAMAMXSAAAAXAMASXSASMASXMASMSAASMSAMAXAXMASMMXMASMSMMMMAXAAXMAXXMXSAMAMSMMMASASMMXXAMASXMAAASMS
AASXSMSSSMMMSAMAMXMAXAMXSXAAXMASXXXXMAMXXSMAMSXMASASMMMSMMXMMXMXSXSXMXXAAMMXMAXSAMMMSXMSXAXMSSXMSSSSSXSSSXMXAMAMMMXMASAMAMMXAAXMASAMAMXMMMXM
AMAMXAMAAMAAXMSXXASMMSMMXMSSSMAMMMSMSSSSMMXAMSASAMMSASXXASAXAXMMXXMASMSMSMSSSXMASXAMAMAMXSXSAMAMAXMAXAXAAAMSMMMSAXXSXSSMASMSAMSSMMMSXMSXMSAM
SMAMMXMXMSMSXXAMSASXAAAAMMAMAMMSSMAAAAXAAMSMXSAMMSXSAMXXASMXSAMAXMASAAAAAAXMASXXXMSAMSMMAXMMASXMASMAMMMMMSMAMSASXSMSAMXSASAMMAAAMAXMAMAAMSXS
MAAMSXMXXAAXMMAXAMXMMSMXXMAMAMMAASXMMSMMSMAMAMAMSAMXXAAMXMAAXAAXMXMASXMXMXMSXMMMMAAMMXXMXSASMMXMASMASAASXMMAMMAMASAXMAXXXMXMSSMMMSSMSSMXMMAM
SSMASAMSSMSMMSMMMMXSMMAMSSSSSSMXXMSSSXMAXMASMMAMAMSMMSXMAMMMSMMSMAXAXXMASXMMAMMAASMXMSSXXXAMAAXMASAMSMXSAMSXMXAMXMMMAMMSSMAAMMASAMAAAXXMASAM
XAMXMAAAAXAAMAXMAMXMAMMMAAAXAXMXSAXXAASMMSXSXSSSSSMMAMASXSMXMAAAXMMMSASASAASMMSSMXXAXAMMSXXSMSSMAMXMXXMMXMASMSASXMXSXSAAASMSXSSMAXMMMXXSAXAS
XXMAMMMMXSXSXSXSASXMAMXMMSMMSMSAMMMMSMMSASXMAAAAMXAMXSAMXSXXXMMMSXAASXMASXMXMMMMMMSMMMSAXXMXXMAMAXMASMMSASASMAMMMSMSAMXSMXMXAMAAXMMXSSMMXSAM
SSXSAAAXAMAXAXMSASXSASXMAMXAXAMSSMAAAAXSMSAMAMMMMXMMMXAAAMMSMMSMMMMMMAMAMMMAMAAAAXASAXMMXXAAMXMMMMSAAAASASXXMAMAMMAMXMAXXXSMXMASMSXAXAAAAMAM
MAAMMSMSMMAMMMASAMAMAAMMASMSMXMAXMMSSMMMAXXMSSMXAASASMSMXMAMAAAMAASMSMMASXSASXMSMXAXXXAMAMSXSAAAAMMSMMMMAMASMMSSXMAMXMAMMXSAMXXXAXMAMSMMSXMM
MMMMXMMAXMSXMMAMAMXMAMAMAXMAMMMMSAMMXAAMMMAAXAXMSMSASAAAAMSMMMSSSXSAAXMXSXSXSMMMXMSMMSXMAMAAXXXMXSAXXSAMXMMMAXAAASXMSMSMSMXXXXSMSMAMXXMAMAAS
XAMAMMSXSMXASMMMAMXXASXMAMSAMAAASASASXMMAXXMSMMMXAMAMXMMMMMAMSAMMAMMMSMASAMMMMASAMXAMAMSXMMSMSSSMMMXMSASXSXSXMMSMMAAXAXAAMMMMMSAMXAMMXMASMMM
SMSASAAASASAMXMSASXAMAAMAXSAXMSXXAMASMSMSMXMAXAAMMSSXAMASXSAMMXMXMMAAAMAMMMAAXASMSSXMAXXAXAAAAAAMASAAMAMMMMAXXMAMSMMMMMSMMAAAAMAMSMSAXMASAXM
SMSASMMMMAMXMAXXXMMXMXXMXMXAMMMMMAMXSXSAAXXSSSMMSAMAMAMXSAMXSAAXAAMSMXMAXSSXSSXMMAMXSXMSMMSMMMXMSASMSMAMAAMXSXXMMAAAAXXMASXSMSMSMMMAMXXAXXMM
MXMAMMAMMAMAMSSMXSAASXSSMSMAMAAAMXMMMAMSMMMMXMASMASMSMMAMAXXAXASMSMXMSSMMXMAXMMMMMMMASAAMAMMASAAMXSAXMMSSXSASAASXXSSSSXAXXAAMMMXASMSMSMSSSMS
XXMAXSASMSSMXAAXAXSASAMMAAMAMMMMXAMAMXMXXMAMMXXXSAMXAASXSMSXMMMAXAASAAAXSMSSMASAMXSAMMMMMAXSAXMSMMMMMXMAMXMAMXSMSAXAMXMXMMSMMSAMAMAXAAAMAXAS
MXXMXXASAMAMSSSMXMAXMAMMSMSMMSASMSSSSSMXSSSSMMMMMMMMSXMAAASMSASASMXMMMSMMAAMXAMASMMMXSAXXAMMASAMAMAAMXMASMMSSXXAMXMAMAMAMAMSAMXMAMAMMMMMAMSM
SAASMMXMAMSMAAAAXXMASAMXAAAAXMASAAAAAMXAMXAAAAAAAAXXXMASMXMASASAMXXXXMAAMMMSMMSAMAAXASMSXMAMAAASAMAMSAMXSXAAAAMMMAAASASASMMMMSAMAMXSSXMMXSAM
MXSAAASMXMAMXSMMMXXASASXMSMSMMXMXMMMAMMASMSMMSSSSSMSAMXXXXMAMXMXMSAMXSSSMSAMXAMXSSMMASAXXAXMMSXSMSAMXAXMSMXSMSMXSSSMSMSASAMXXSXSXSAMXSAMXMAS ";
