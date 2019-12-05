use super::*;

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn sides(&self) -> (u32, u32, u32) {
        (self.length * self.width, self.width * self.height, self.height * self.length)
    }

    fn wrapping_paper(&self) -> u32 {
        let (a, b, c) = self.sides();
        2 * (self.length * self.width + self.width * self.height + self.height * self.length)
            + cmp::min(a, cmp::min(b, c))
    }

    fn ribbon(&self) -> u32 {
        let max = cmp::max(self.length, cmp::max(self.width, self.height));
        (self.length + self.width + self.height - max) * 2 + self.length * self.width * self.height
    }
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Present>> {
    let present = chain((parser::u32(), token('x'), parser::u32(), token('x'), parser::u32()))
        .map(|(length, _, width, _, height)| Present { length, width, height });
    present.collect_sep_by(token('\n'))
}

fn part1(presents: &[Present]) -> u32 {
    presents.iter().map(|p| p.wrapping_paper()).sum()
}

fn part2(presents: &[Present]) -> u32 {
    presents.iter().map(|p| p.ribbon()).sum()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 2).await?;
    let presents = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&presents), 1_606_483);
    assert_eq!(part2(&presents), 3842356);
    Ok(())
}
