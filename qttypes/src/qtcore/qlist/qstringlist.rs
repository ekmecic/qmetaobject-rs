use std::{fmt, ops::Index};

use cpp::{cpp, cpp_class};

use crate::QString;

cpp_class!(
    /// Wrapper around [`QStringList`][class] class.
    ///
    /// [class]: https://doc.qt.io/qt-5/qstringlist.html
    #[derive(Default, Clone, PartialEq, Eq)]
    pub unsafe struct QStringList as "QStringList"
);
impl QStringList {
    pub fn new() -> QStringList {
        cpp!(unsafe [] -> QStringList as "QStringList" {
            return QStringList();
        })
    }

    pub fn insert(&mut self, index: usize, value: QString) {
        cpp!(unsafe [self as "QStringList*", index as "size_t", value as "QString"] {
            self->insert(index, value);
        });
    }

    pub fn push(&mut self, value: QString) {
        cpp!(unsafe [self as "QStringList*", value as "QString"] {
           self->append(value);
        });
    }

    pub fn clear(&mut self) {
        cpp!(unsafe [self as "QStringList*"] {
            self->clear();
        });
    }

    pub fn remove(&mut self, index: usize) {
        cpp!(unsafe [self as "QStringList*", index as "size_t"] {
            self->removeAt(index);
        })
    }

    pub fn len(&self) -> usize {
        cpp!(unsafe [self as "QStringList*"] -> usize as "size_t" { return self->size(); })
    }
}

impl Index<usize> for QStringList {
    type Output = QString;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            &*cpp!([self as "QStringList*", index as "size_t"] -> *const QString as "const QString*" {
                return &(*self)[index];
            })
        }
    }
}

impl fmt::Debug for QStringList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut temp = f.debug_list();
        for i in 0..self.len() {
            temp.entry(&self[i]);
        }
        temp.finish()
    }
}

impl<T, const N: usize> From<[T; N]> for QStringList
where
    QString: From<T>,
{
    fn from(s: [T; N]) -> Self {
        let mut list = QStringList::new();
        for i in s {
            list.push(QString::from(i));
        }
        list
    }
}

impl<T> From<Vec<T>> for QStringList
where
    QString: From<T>,
{
    fn from(s: Vec<T>) -> Self {
        let mut list = QStringList::new();
        for i in s {
            list.push(QString::from(i));
        }
        list
    }
}

impl<T> From<&[T]> for QStringList
where
    T: Clone,
    QString: From<T>,
{
    fn from(s: &[T]) -> Self {
        let mut list = QStringList::new();
        for i in s {
            list.push(QString::from(i.clone())); // i: &T in case of slice.
        }
        list
    }
}

impl<T> From<QStringList> for Vec<T>
where
    T: Clone,
    QString: Into<T>,
{
    fn from(arr: QStringList) -> Self {
        let mut v = Vec::with_capacity(arr.len());
        for i in 0..arr.len() {
            v.push(arr[i].clone().into());
        }
        v
    }
}

#[test]
fn test_qstringlist() {
    let mut qstringlist = QStringList::new();
    qstringlist.push("One".into());
    qstringlist.push("Two".into());

    assert_eq!(qstringlist.len(), 2);
    assert_eq!(qstringlist[0], QString::from("One"));

    qstringlist.remove(0);
    assert_eq!(qstringlist[0], QString::from("Two"));

    qstringlist.insert(0, "Three".into());
    assert_eq!(qstringlist[0], QString::from("Three"));

    assert_eq!(qstringlist, QStringList::from(["Three", "Two"]));
    assert_eq!(qstringlist, QStringList::from(["Three".to_string(), "Two".to_string()]));
    assert_eq!(qstringlist, QStringList::from([QString::from("Three"), QString::from("Two")]));

    assert_eq!(qstringlist, QStringList::from(vec!["Three", "Two"]));
    assert_eq!(qstringlist, QStringList::from(vec!["Three".to_string(), "Two".to_string()]));
    assert_eq!(qstringlist, QStringList::from(vec![QString::from("Three"), QString::from("Two")]));

    let t = ["Three", "Two"];
    assert_eq!(qstringlist, QStringList::from(t));
    let t = ["Three".to_string(), "Two".to_string()];
    assert_eq!(qstringlist, QStringList::from(t));
    let t = [QString::from("Three"), QString::from("Two")];
    assert_eq!(qstringlist, QStringList::from(t));

    let temp: Vec<String> = qstringlist.clone().into();
    assert_eq!(temp, vec!["Three".to_string(), "Two".to_string()]);
    let temp: Vec<QString> = qstringlist.clone().into();
    assert_eq!(temp, vec![QString::from("Three"), QString::from("Two")]);

    qstringlist.clear();
    assert_eq!(qstringlist.len(), 0);
}