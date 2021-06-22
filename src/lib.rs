mod field {
    struct Cell { row: i32, col: i32 }

    impl Cell {
        fn is_vertical_with(&self, other: &Cell) -> bool {
            return self.row == other.row;
        }

        fn is_horizontal_with(&self, other: &Cell) -> bool {
            return self.col == other.col;
        }
    }

    struct Lane { start: Cell, end: Cell }

    impl Lane {
        fn is_vertical(&self) -> bool {
            return self.start.is_vertical_with(&self.end);
        }

        fn is_horizontal(&self) -> bool {
            return self.start.is_horizontal_with(&self.end);
        }
    }

    #[test]
    fn test_vertical() {
        let c1 = Cell{row: 0, col: 0};
        let c2 = Cell{row: 0, col: 3};
        let lane = Lane{start: c1, end: c2};
        assert!(lane.is_vertical());
        assert!(!lane.is_horizontal());
    }

    #[test]
    fn test_horizontal() {
        let c1 = Cell{row: 0, col: 0};
        let c2 = Cell{row: 3, col: 0};
        let lane = Lane{start: c1, end: c2};
        assert!(!lane.is_vertical());
        assert!(lane.is_horizontal());
    }

}
