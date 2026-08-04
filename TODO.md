# TODO

Known gaps in dtk-rs, roughly in priority order.

## Blocking deepin-liferaft RIIR

(none — all APIs liferaft needs are bound)

## Coverage gaps (see GEN_REPORT.md for per-method detail)

- [ ] Qt container types: `QList<T>`/`QVector<T>`/`QMap`/`QVariant` in signatures
      (e.g. `DDrawerGroup::addWidget`, `DSimpleListView` item lists). Map to `Vec<T>` + shim conversion.
- [ ] Qt classes beyond `QWidget` as param/return: `QBoxLayout*`, `QAction*`, `QPrinter*`,
      `QAbstractItemModel*`... Currently only `QWidget` crosses the two cxx bridges;
      the others are skipped. Fix: declare each in both bridges and cast like `QWidget`.
- [ ] Multi-line method signatures (~30 parse failures): make gen.py join
      continuation lines before matching.
- [ ] Non-exported nested classes (`DPrinter` inside dprintpreviewwidget.h etc.):
      parser registers their enums but can't bind the classes themselves.
- [ ] `Q_ENUM`-exposed enum *values*: we map enums to bare i32 today and hand-maintain
      `dtk::qt` constants. gen.py could scrape enum bodies and emit Rust consts automatically.

## Libraries

- [ ] DGui bindings (`/usr/include/dtk6/DGui`) — same generator pass, second output module.
- [ ] DCore bindings (`/usr/include/dtk6/DCore`) — DConfig, DStandardPaths, DLog...

## Hygiene

- [ ] Value-type wrappers (QColor/QFont/QPalette/QPixmap/QSize/QPoint/QRect, QIcon)
      intentionally leak today. Add `Drop` calling a shim `delete` fn.
- [ ] Callback registry never frees entries; signals connected on long-lived objects
      accumulate. Add an unregister path if it ever shows up in profiles.
- [ ] `qtptr` cross-bridge cast (`as _`) works because the opaque types are
      layout-identical; fine in practice, worth a comment-level audit if cxx changes.
