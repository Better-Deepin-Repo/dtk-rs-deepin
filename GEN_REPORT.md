# DTK6 widget binding coverage report

classes: 60, methods generated: 416, skipped: 156

## DAbstractDialog — 8 methods generated, 2 skipped
- `inline void move(int x, int y)` ← unsupported return type: inline void
- `inline void setGeometry(int x, int y, int width, int height)` ← unsupported return type: inline void

## DAccessibilityChecker — 4 methods generated, 0 skipped

## DAlertControl — 9 methods generated, 1 skipped
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← unsupported param type: QWidget

## DArrowButton — 3 methods generated, 0 skipped

## DArrowLineDrawer — 2 methods generated, 0 skipped

## DArrowRectangle — 38 methods generated, 1 skipped
- `void setContent(QWidget *content);` ← unsupported param type: QWidget

## DBackgroundGroup — 3 methods generated, 5 skipped
- `QMargins itemMargins() const;` ← unsupported return type: QMargins
- `void setLayout(QLayout *layout);` ← unsupported param type: QLayout
- `void setBackgroundRole(QPalette::ColorRole role);` ← unsupported param type: QPalette::ColorRole
- `QPalette::ColorRole backgroundRole() const;` ← unsupported return type: QPalette::ColorRole
- `void setItemMargins(QMargins itemMargins);` ← unsupported param type: QMargins

## DBaseLine — 4 methods generated, 4 skipped
- `void setLeftContent(QWidget *content);` ← unsupported param type: QWidget
- `void setRightContent(QWidget *content);` ← unsupported param type: QWidget
- `QBoxLayout *leftLayout();` ← unsupported return type: QBoxLayout *
- `QBoxLayout *rightLayout();` ← unsupported return type: QBoxLayout *

## DBlurEffectWidget — 19 methods generated, 3 skipped
- `void setMaskPath(const QPainterPath &path);` ← unsupported param type: const QPainterPath
- `void setSourceImage(const QImage &image, bool autoScale = true);` ← unsupported param type: const QImage
- `void updateBlurSourceImage(const QRegion &ren);` ← unsupported param type: const QRegion

## DCircleProgress — 10 methods generated, 2 skipped
- `QLabel *topLabel();` ← unsupported return type: QLabel *
- `QLabel *bottomLabel();` ← unsupported return type: QLabel *

## DColoredProgressBar — 1 methods generated, 2 skipped
- `void addThreshold(int threshold, QBrush brush);` ← unsupported param type: QBrush
- `QList<int> thresholds() const;` ← unsupported return type: QList<int>

## DComboBox — 1 methods generated, 1 skipped
- `virtual bool eventFilter(QObject *watched, QEvent *event) override;` ← unsupported param type: QObject

## DCrumbTextFormat — 8 methods generated, 2 skipped
- `QBrush background() const;` ← unsupported return type: QBrush
- `void setBackground(const QBrush &background);` ← unsupported param type: const QBrush

## DCrumbEdit — 11 methods generated, 6 skipped
- `bool insertCrumb(const DCrumbTextFormat &format, int pos = -1);` ← unsupported param type: const DCrumbTextFormat
- `bool appendCrumb(const DCrumbTextFormat &format);` ← unsupported param type: const DCrumbTextFormat
- `QStringList crumbList() const;` ← unsupported return type: QStringList
- `DCrumbTextFormat crumbTextFormat(const QString &text) const;` ← unsupported return type: DCrumbTextFormat
- `DCrumbTextFormat makeTextFormat() const;` ← unsupported return type: DCrumbTextFormat
- `DCrumbTextFormat makeTextFormat(CrumbType type) const;` ← unsupported return type: DCrumbTextFormat

## DDrawer — 7 methods generated, 3 skipped
- `void setHeader(QWidget *header);` ← unsupported param type: QWidget
- `void setContent(QWidget *content, Qt::Alignment alignment = Qt::AlignHCenter);` ← unsupported param type: QWidget
- `void setAnimationEasingCurve(QEasingCurve curve);` ← unsupported param type: QEasingCurve

## DDrawerGroup — 3 methods generated, 5 skipped
- `QList<DDrawer *> expands() const;` ← unsupported return type: QList<DDrawer *>
- `void addExpand(DDrawer *expand, int id = -1);` ← unsupported param type: DDrawer
- `void setId(DDrawer *expand, int id);` ← unsupported param type: DDrawer
- `void removeExpand(DDrawer *expand);` ← unsupported param type: DDrawer
- `int id(DDrawer *expand) const;` ← unsupported param type: DDrawer

## DFileChooserEdit — 3 methods generated, 8 skipped
- `void setFileMode(QFileDialog::FileMode mode);` ← unsupported param type: QFileDialog::FileMode
- `QFileDialog::FileMode fileMode() const;` ← unsupported return type: QFileDialog::FileMode
- `void setNameFilters(const QStringList &filters);` ← unsupported param type: const QStringList
- `QStringList nameFilters() const;` ← unsupported return type: QStringList
- `void setDirectoryUrl(const QUrl &directory);` ← unsupported param type: const QUrl
- `QUrl directoryUrl();` ← unsupported return type: QUrl
- `void setFileDialog(QFileDialog *fileDialog);` ← unsupported param type: QFileDialog
- `QFileDialog *fileDialog() const;` ← unsupported return type: QFileDialog *

## DFileDialog — 5 methods generated, 7 skipped
- `explicit DFileDialog(QWidget *parent = Q_NULLPTR,` ← signature parse failed
- `const QString &caption = QString(),` ← signature parse failed
- `const QString &directory = QString(),` ← signature parse failed
- `const QString &filter = QString());` ← signature parse failed
- `void addComboBox(const QString &text, const QStringList &data);` ← unsupported param type: const QStringList
- `void addComboBox(const QString &text, const DComboBoxOptions &options);` ← unsupported param type: const DComboBoxOptions
- `void addLineEdit(const QString &text, const DLineEditOptions &options);` ← unsupported param type: const DLineEditOptions

## DFontComboBox — 3 methods generated, 4 skipped
- `void setWritingSystem(QFontDatabase::WritingSystem);` ← unsupported param type: QFontDatabase::WritingSystem
- `QFontDatabase::WritingSystem writingSystem() const;` ← unsupported return type: QFontDatabase::WritingSystem
- `void setFontFilters(QFontComboBox::FontFilters filters);` ← unsupported param type: QFontComboBox::FontFilters
- `QFontComboBox::FontFilters fontFilters() const;` ← unsupported return type: QFontComboBox::FontFilters

## DGraphicsGlowEffect — 0 methods generated, 15 skipped
- `inline void setOffset(qreal dx, qreal dy) {m_xOffset = dx; m_yOffset = dy;}` ← signature parse failed
- `inline void setXOffset(qreal dx) {m_xOffset = dx;}` ← signature parse failed
- `inline qreal xOffset() const {return m_xOffset;}` ← signature parse failed
- `inline void setYOffset(qreal dy) {m_yOffset = dy;}` ← signature parse failed
- `inline qreal yOffset() const {return m_yOffset;}` ← signature parse failed
- `inline void setDistance(qreal distance) { m_distance = distance; updateBoundingR` ← signature parse failed
- `inline qreal distance() const { return m_distance; }` ← signature parse failed
- `inline void setBlurRadius(qreal blurRadius) { m_blurRadius = blurRadius; updateB` ← signature parse failed
- `inline qreal blurRadius() const { return m_blurRadius; }` ← signature parse failed
- `inline void setColor(const QColor &color) { m_color = color; }` ← signature parse failed
- `inline QColor color() const { return m_color; }` ← signature parse failed
- `inline qreal opacity() const { return m_opacity; }` ← signature parse failed
- `inline void setOpacity(qreal opacity) { m_opacity = opacity; }` ← signature parse failed
- `void draw(QPainter *painter);` ← unsupported param type: QPainter
- `QRectF boundingRectFor(const QRectF &rect) const;` ← unsupported return type: QRectF

## DHeaderLine — 2 methods generated, 1 skipped
- `void setContent(QWidget *content);` ← unsupported param type: QWidget

## DImageViewer — 20 methods generated, 3 skipped
- `QImage image() const;` ← unsupported return type: QImage
- `void setImage(const QImage &image);` ← unsupported param type: const QImage
- `Q_SLOT void scaleAtPoint(QPoint pos, qreal factor);` ← unsupported return type: Q_SLOT void

## DIpv4LineEdit — 9 methods generated, 0 skipped

## DKeySequenceEdit — 2 methods generated, 3 skipped
- `bool setKeySequence(const QKeySequence &keySequence);` ← unsupported param type: const QKeySequence
- `QKeySequence keySequence();` ← unsupported return type: QKeySequence
- `QString getKeySequence(QKeySequence sequence);` ← unsupported param type: QKeySequence

## DLineEdit — 27 methods generated, 6 skipped
- `QLineEdit *lineEdit() const;` ← unsupported return type: QLineEdit *
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← unsupported param type: QWidget
- `void setLeftWidgets(const QList<QWidget *> &list);` ← unsupported param type: const QList<QWidget *>
- `void setRightWidgets(const QList<QWidget *> &list);` ← unsupported param type: const QList<QWidget *>
- `QLineEdit::EchoMode echoMode() const;` ← unsupported return type: QLineEdit::EchoMode
- `void setEchoMode(QLineEdit::EchoMode mode);` ← unsupported param type: QLineEdit::EchoMode

## DListView — 17 methods generated, 15 skipped
- `/// return true if rect intersects contentsVisualRect+qMax(cacheBuffer,cacheCoun` ← signature parse failed
- `State state() const;` ← unsupported return type: State
- `void setModel(QAbstractItemModel *model);` ← unsupported param type: QAbstractItemModel
- `DStyledItemDelegate::BackgroundType backgroundType() const;` ← unsupported return type: DStyledItemDelegate::BackgroundType
- `QMargins itemMargins() const;` ← unsupported return type: QMargins
- `bool addItem(const QVariant &data);` ← unsupported param type: const QVariant
- `bool addItems(const QVariantList &datas);` ← unsupported param type: const QVariantList
- `bool insertItem(int index, const QVariant &data);` ← unsupported param type: const QVariant
- `bool insertItems(int index, const QVariantList &datas);` ← unsupported param type: const QVariantList
- `int addHeaderWidget(QWidget *widget);` ← unsupported param type: QWidget
- `int addFooterWidget(QWidget *widget);` ← unsupported param type: QWidget
- `void setOrientation(QListView::Flow flow, bool wrapping);` ← unsupported param type: QListView::Flow
- `void edit(const QModelIndex &index);` ← unsupported param type: const QModelIndex
- `void setBackgroundType(DStyledItemDelegate::BackgroundType backgroundType);` ← unsupported param type: DStyledItemDelegate::BackgroundType
- `void setItemMargins(const QMargins &itemMargins);` ← unsupported param type: const QMargins

## DLoadingIndicator — 18 methods generated, 4 skipped
- `QEasingCurve::Type aniEasingType() const;` ← unsupported return type: QEasingCurve::Type
- `void setAniEasingCurve(const QEasingCurve & easing);` ← unsupported param type: const QEasingCurve &
- `void setRotate(QVariant angle);` ← unsupported param type: QVariant
- `void setAniEasingType(QEasingCurve::Type aniEasingType);` ← unsupported param type: QEasingCurve::Type

## DMPRISControl — 3 methods generated, 0 skipped

## DPageIndicator — 16 methods generated, 0 skipped

## DPasswordEdit — 3 methods generated, 1 skipped
- `void setEchoMode(QLineEdit::EchoMode mode);` ← unsupported param type: QLineEdit::EchoMode

## DPictureSequenceView — 7 methods generated, 3 skipped
- `void setPictureSequence(const QString &srcFormat, const QPair<int, int> &range, ` ← unsupported param type: const QPair<int, int>
- `void setPictureSequence(const QStringList &sequence, const bool autoScale = fals` ← unsupported param type: const QStringList
- `void setPictureSequence(const QList<QPixmap> &sequence, const bool autoScale = f` ← unsupported param type: const QList<QPixmap>

## DPrintPreviewWidget — 47 methods generated, 6 skipped
- `void setPageRange(const QVector<int> &rangePages);` ← unsupported param type: const QVector<int>
- `void setColorMode(const DPrinter::ColorMode &colorMode);` ← unsupported param type: const DPrinter::ColorMode
- `void setOrientation(const DPrinter::Orientation &pageOrientation);` ← unsupported param type: const DPrinter::Orientation
- `DPrinter::ColorMode getColorMode();` ← unsupported return type: DPrinter::ColorMode
- `void setWaterMargImage(const QImage &image);` ← unsupported param type: const QImage
- `void themeTypeChanged(DGuiApplicationHelper::ColorType themeType);` ← unsupported param type: DGuiApplicationHelper::ColorType

## DSearchComboBox — 1 methods generated, 0 skipped

## DSearchEdit — 7 methods generated, 0 skipped

## DSettingsDialog — 6 methods generated, 2 skipped
- `void updateSettings(DTK_CORE_NAMESPACE::DSettings *settings);` ← unsupported param type: DTK_CORE_NAMESPACE::DSettings
- `void updateSettings(const QByteArray &translateContext, DTK_CORE_NAMESPACE::DSet` ← unsupported param type: DTK_CORE_NAMESPACE::DSettings

## DSettingsWidgetFactory — 0 methods generated, 7 skipped
- `QPair<QWidget*, QWidget*> createItem(QPointer<DTK_CORE_NAMESPACE::DSettingsOptio` ← signature parse failed
- `QPair<QWidget*, QWidget*> createItem(const QByteArray &translateContext, QPointe` ← signature parse failed
- `static QPair<QWidget*, QWidget*> createStandardItem(const QByteArray &translateC` ← signature parse failed
- `void registerWidget(const QString &viewType, std::function<WidgetCreateHandler> ` ← unsupported param type: std::function<WidgetCreateHandler>
- `void registerWidget(const QString &viewType, std::function<ItemCreateHandler> ha` ← unsupported param type: std::function<ItemCreateHandler>
- `QWidget *createWidget(QPointer<DTK_CORE_NAMESPACE::DSettingsOption> option);` ← unsupported param type: QPointer<DTK_CORE_NAMESPACE::DSettingsOption>
- `QWidget *createWidget(const QByteArray &translateContext, QPointer<DTK_CORE_NAME` ← unsupported param type: QPointer<DTK_CORE_NAMESPACE::DSettingsOption>

## DShadowLine — 1 methods generated, 0 skipped

## DSimpleListItem — 0 methods generated, 3 skipped
- `virtual bool sameAs(DSimpleListItem *item)=0;` ← unsupported param type: DSimpleListItem
- `virtual void drawBackground(QRect rect, QPainter *painter, int index, bool isSel` ← unsupported param type: QPainter
- `virtual void drawForeground(QRect rect, QPainter *painter, int column, int index` ← unsupported param type: QPainter

## DSimpleListView — 25 methods generated, 10 skipped
- `* \algorithms a list of SortAlgorithm, SortAlgorithm is function pointer, it's t` ← signature parse failed
- `* \algorithm the search algorithm, it's type is: 'bool (*) (const DSimpleListIte` ← signature parse failed
- `void setColumnTitleInfo(QList<QString> titles, QList<int> widths, int height);` ← unsupported param type: QList<QString>
- `void setColumnHideFlags(QList<bool> toggleHideFlags, int alwaysVisibleColumn=-1)` ← unsupported param type: QList<bool>
- `void setColumnSortingAlgorithms(QList<SortAlgorithm> *algorithms, int sortColumn` ← unsupported param type: QList<SortAlgorithm>
- `void setSearchAlgorithm(SearchAlgorithm algorithm);` ← unsupported param type: SearchAlgorithm
- `void addItems(QList<DSimpleListItem*> items);` ← unsupported param type: QList<DSimpleListItem*>
- `void addSelections(QList<DSimpleListItem*> items, bool recordLastSelection=true)` ← unsupported param type: QList<DSimpleListItem*>
- `QList<DSimpleListItem*> getSelections();` ← unsupported return type: QList<DSimpleListItem*>
- `void refreshItems(QList<DSimpleListItem*> items);` ← unsupported param type: QList<DSimpleListItem*>

## DSizeModeHelper — 0 methods generated, 2 skipped
- `return DGUI_NAMESPACE::DGuiApplicationHelper::isCompactMode() ? t1 : t2;` ← signature parse failed
- `static inline T element(const T &t1, const T &t2)` ← unsupported return type: inline T

## DSlider — 18 methods generated, 7 skipped
- `QSlider *slider();` ← unsupported return type: QSlider *
- `void setLeftTicks(const QStringList &info);` ← unsupported param type: const QStringList
- `void setRightTicks(const QStringList &info);` ← unsupported param type: const QStringList
- `void setAboveTicks(const QStringList &info);` ← unsupported param type: const QStringList
- `void setBelowTicks(const QStringList &info);` ← unsupported param type: const QStringList
- `void setMarkPositions(QList<int> list);` ← unsupported param type: QList<int>
- `QSlider::TickPosition tickPosition() const;` ← unsupported return type: QSlider::TickPosition

## DSpinBox — 4 methods generated, 2 skipped
- `QLineEdit *lineEdit() const;` ← unsupported return type: QLineEdit *
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← unsupported param type: QWidget

## DDoubleSpinBox — 4 methods generated, 2 skipped
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← unsupported param type: QWidget
- `QLineEdit *lineEdit() const;` ← unsupported return type: QLineEdit *

## DSpinner — 4 methods generated, 0 skipped

## DSwitchButton — 1 methods generated, 0 skipped

## DSwitchLineExpand — 3 methods generated, 0 skipped

## DTabletWindowOptionButton — 1 methods generated, 0 skipped

## DTickEffect — 6 methods generated, 0 skipped

## DTipLabel — 1 methods generated, 1 skipped
- `void setForegroundRole(DPalette::ColorType color);` ← unsupported param type: DPalette::ColorType

## DTitlebarToolBaseInterface — 3 methods generated, 2 skipped
- `explicit DTitlebarToolBaseInterface(QObject *parent = nullptr) : QObject(parent)` ← signature parse failed
- `virtual ~DTitlebarToolBaseInterface(){}` ← signature parse failed

## DTitleBarToolInterface — 1 methods generated, 2 skipped
- `explicit DTitleBarToolInterface(QObject *parent = nullptr) : DTitlebarToolBaseIn` ← signature parse failed
- `virtual ~DTitleBarToolInterface(){}` ← signature parse failed

## DTitleBarSpacerInterface — 2 methods generated, 2 skipped
- `explicit DTitleBarSpacerInterface(QObject *parent = nullptr) : DTitlebarToolBase` ← signature parse failed
- `virtual ~DTitleBarSpacerInterface(){}` ← signature parse failed

## DTitlebarSettings — 1 methods generated, 1 skipped
- `bool initilize(QList<DTitlebarToolBaseInterface *> &tools, const QString &path);` ← unsupported param type: QList<DTitlebarToolBaseInterface *>

## DToolButton — 2 methods generated, 0 skipped

## DWaterProgress — 5 methods generated, 0 skipped

## DWindowCloseButton — 1 methods generated, 0 skipped

## DWindowMaxButton — 3 methods generated, 0 skipped

## DWindowMinButton — 1 methods generated, 0 skipped

## DWindowOptionButton — 1 methods generated, 0 skipped

## DWindowQuitFullButton — 1 methods generated, 0 skipped
