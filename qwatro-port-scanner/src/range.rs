use crate::error::ScannerError;
use std::{
    fmt::{Debug, Display, Formatter},
    num::NonZeroU16,
};

/// Диапазон портов для сканирования
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PortRange {
    Ordered(Vec<NonZeroU16>),
    Specific(Vec<NonZeroU16>),
}

impl PortRange {
    /// Упорядоченный диапазон портов (`min..max`).
    /// Возвращает ошибку `ScannerError::PortRangeMinGreaterThanMax` в случае, если начало диапазона
    /// больше, чем его конечное значение.
    pub fn ordered(min: NonZeroU16, max: NonZeroU16) -> Result<Self, ScannerError> {
        if min > max {
            return Err(ScannerError::PortRangeMinGreaterThanMax);
        }

        // let ports = Vec::with_capacity((max.get() - min.get() + 1) as usize);
        // for p in min

        Ok(Self::Ordered(
            (min.get()..=max.get())
                .map(|v| v.try_into().expect("port must be NonZeroU16"))
                .collect(),
        ))
    }

    /// Специфический набор портов
    pub fn specific(ports: Vec<NonZeroU16>) -> Result<Self, ScannerError> {
        Ok(Self::Specific(ports))
    }
}

impl IntoIterator for PortRange {
    type Item = NonZeroU16;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            PortRange::Ordered(ports) => ports.into_iter(),
            PortRange::Specific(ports) => ports.into_iter(),
        }
    }
}

impl Display for PortRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PortRange::Ordered(ports) => match (ports.first(), ports.last()) {
                (Some(first), Some(last)) => write!(f, "({} - {})", first, last),
                _ => write!(f, "(invalid)"),
            },
            PortRange::Specific(ports) => write!(
                f,
                "({})",
                ports
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;

    use crate::error::ScannerError;
    use crate::range::PortRange;

    #[test]
    fn ordered() {
        let range = PortRange::ordered(nzu16(100), nzu16(105)).unwrap();

        assert_eq!(
            range.into_iter().collect::<Vec<_>>(),
            vec![
                nzu16(100),
                nzu16(101),
                nzu16(102),
                nzu16(103),
                nzu16(104),
                nzu16(105)
            ]
        );
    }

    #[test]
    fn ordered_invalid_range() {
        assert_eq!(
            PortRange::ordered(nzu16(105), nzu16(100)),
            Err(ScannerError::PortRangeMinGreaterThanMax)
        );
    }

    #[test]
    fn specific() {
        let range = PortRange::specific(vec![nzu16(1000), nzu16(2000), nzu16(3000)]).unwrap();

        assert_eq!(
            range.into_iter().collect::<Vec<_>>(),
            vec![nzu16(1000), nzu16(2000), nzu16(3000)]
        );
    }

    fn nzu16(v: u16) -> NonZeroU16 {
        NonZeroU16::try_from(v).expect("invalid value")
    }
}
