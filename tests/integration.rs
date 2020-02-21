#![allow(dead_code)]
#![allow(unused_imports)]

extern crate static_assertions as sa;

struct Database;
struct DatabaseMock;
struct MockDatabase;

struct ImageCache;
struct ImageCacheMock;
struct MockImageCache;

struct Analytics;
struct DummyAnalytics;
struct AnalyticsMock;
struct MockAnalytics;

mod appended {
    use super::{
        Database, 
        DatabaseMock,
        ImageCache,
        ImageCacheMock,
        Analytics,
        DummyAnalytics,
        AnalyticsMock,
    };

    mod attribute {
        use test_double::test_double;

        #[test_double]
        use super::{
            Database, 
            ImageCache
        };

        sa::assert_type_eq_all!(Database, super::DatabaseMock);
        sa::assert_type_eq_all!(ImageCache, super::ImageCacheMock);
    }

    mod attribute_alt_name {
        use test_double::test_double;

        #[test_double((DummyAnalytics))]
        use super::Analytics;

        sa::assert_type_eq_all!(Analytics, super::DummyAnalytics);
    }

    mod functionlike {
        use test_double::test_doubles;

        test_doubles! {
            use super::Database;
            use super::ImageCache;
        }

        sa::assert_type_eq_all!(Database, super::DatabaseMock);
        sa::assert_type_eq_all!(ImageCache, super::ImageCacheMock);
    }
}

mod prefixed {
    use super::{
        Database, 
        MockDatabase,
        ImageCache,
        MockImageCache,
        Analytics,
        DummyAnalytics,
        MockAnalytics,
    };

    mod attribute_prefixed {
        use test_double::test_double_prefixed;

        #[test_double_prefixed]
        use super::{
            Database, 
            ImageCache
        };

        sa::assert_type_eq_all!(Database, super::MockDatabase);
        sa::assert_type_eq_all!(ImageCache, super::MockImageCache);
    }

    mod attribute_alt_name {
        use test_double::test_double_prefixed;

        #[test_double_prefixed((DummyAnalytics))]
        use super::Analytics;

        sa::assert_type_eq_all!(Analytics, super::DummyAnalytics);
    }

    mod functionlike {
        use test_double::test_doubles_prefixed;

        test_doubles_prefixed! {
            use super::{Database, ImageCache};
        }

        sa::assert_type_eq_all!(Database, super::MockDatabase);
        sa::assert_type_eq_all!(ImageCache, super::MockImageCache);
    }
}



