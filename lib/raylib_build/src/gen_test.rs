
#[cfg(test)]
mod tests {

    include!("../resources/out/atlas.rs");

    use super::*;
    #[test]
    fn atlas() {
        let d = DownStand;
        assert_eq!("", d.asset());
    }
}