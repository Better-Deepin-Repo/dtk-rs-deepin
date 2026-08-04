# DTK6 widget binding 覆盖报告

类: 60, 已生成方法: 320, 跳过: 252

## DAbstractDialog — 2 方法已生成, 8 跳过
- `DisplayPosition displayPosition() const;` ← 返回类型不支持: DisplayPosition
- `void move(const QPoint &pos);` ← 参数类型不支持: const QPoint
- `inline void move(int x, int y)` ← 返回类型不支持: inline void
- `void setGeometry(const QRect &rect);` ← 参数类型不支持: const QRect
- `inline void setGeometry(int x, int y, int width, int height)` ← 返回类型不支持: inline void
- `void moveToCenterByRect(const QRect &rect);` ← 参数类型不支持: const QRect
- `void moveToTopRightByRect(const QRect &rect);` ← 参数类型不支持: const QRect
- `void setDisplayPosition(DisplayPosition displayPosition);` ← 参数类型不支持: DisplayPosition

## DAccessibilityChecker — 2 方法已生成, 2 跳过
- `void setOutputFormat(OutputFormat format);` ← 参数类型不支持: OutputFormat
- `OutputFormat outputFormat() const;` ← 返回类型不支持: OutputFormat

## DAlertControl — 4 方法已生成, 6 跳过
- `void setAlertColor(QColor c);` ← 参数类型不支持: QColor
- `QColor alertColor() const;` ← 返回类型不支持: QColor
- `QColor defaultAlertColor() const;` ← 返回类型不支持: QColor
- `void setMessageAlignment(Qt::Alignment alignment);` ← 参数类型不支持: Qt::Alignment
- `Qt::Alignment messageAlignment() const;` ← 返回类型不支持: Qt::Alignment
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← 参数类型不支持: QWidget

## DArrowButton — 2 方法已生成, 1 跳过
- `void setArrowDirection(ArrowDirection direction);` ← 参数类型不支持: ArrowDirection

## DArrowLineDrawer — 2 方法已生成, 0 跳过

## DArrowRectangle — 30 方法已生成, 9 跳过
- `QColor borderColor() const;` ← 返回类型不支持: QColor
- `QColor backgroundColor() const;` ← 返回类型不支持: QColor
- `ArrowDirection arrowDirection() const;` ← 返回类型不支持: ArrowDirection
- `void setBorderColor(const QColor &borderColor);` ← 参数类型不支持: const QColor
- `void setBackgroundColor(const QColor &backgroundColor);` ← 参数类型不支持: const QColor
- `void setBackgroundColor(DBlurEffectWidget::MaskColorType type);` ← 参数类型不支持: DBlurEffectWidget::MaskColorType
- `void setArrowDirection(ArrowDirection value);` ← 参数类型不支持: ArrowDirection
- `void setContent(QWidget *content);` ← 参数类型不支持: QWidget
- `QSize getFixedSize();` ← 返回类型不支持: QSize

## DBackgroundGroup — 3 方法已生成, 5 跳过
- `QMargins itemMargins() const;` ← 返回类型不支持: QMargins
- `void setLayout(QLayout *layout);` ← 参数类型不支持: QLayout
- `void setBackgroundRole(QPalette::ColorRole role);` ← 参数类型不支持: QPalette::ColorRole
- `QPalette::ColorRole backgroundRole() const;` ← 返回类型不支持: QPalette::ColorRole
- `void setItemMargins(QMargins itemMargins);` ← 参数类型不支持: QMargins

## DBaseLine — 4 方法已生成, 4 跳过
- `void setLeftContent(QWidget *content);` ← 参数类型不支持: QWidget
- `void setRightContent(QWidget *content);` ← 参数类型不支持: QWidget
- `QBoxLayout *leftLayout();` ← 返回类型不支持: QBoxLayout *
- `QBoxLayout *rightLayout();` ← 返回类型不支持: QBoxLayout *

## DBlurEffectWidget — 12 方法已生成, 10 跳过
- `BlurMode mode() const;` ← 返回类型不支持: BlurMode
- `BlendMode blendMode() const;` ← 返回类型不支持: BlendMode
- `QColor maskColor() const;` ← 返回类型不支持: QColor
- `void setMaskPath(const QPainterPath &path);` ← 参数类型不支持: const QPainterPath
- `void setSourceImage(const QImage &image, bool autoScale = true);` ← 参数类型不支持: const QImage
- `void setMode(BlurMode mode);` ← 参数类型不支持: BlurMode
- `void setBlendMode(BlendMode blendMode);` ← 参数类型不支持: BlendMode
- `void setMaskColor(QColor maskColor);` ← 参数类型不支持: QColor
- `void setMaskColor(MaskColorType type);` ← 参数类型不支持: MaskColorType
- `void updateBlurSourceImage(const QRegion &ren);` ← 参数类型不支持: const QRegion

## DCircleProgress — 6 方法已生成, 6 跳过
- `const QColor backgroundColor() const;` ← 返回类型不支持: const QColor
- `void setBackgroundColor(const QColor &color);` ← 参数类型不支持: const QColor
- `const QColor chunkColor() const;` ← 返回类型不支持: const QColor
- `void setChunkColor(const QColor &color);` ← 参数类型不支持: const QColor
- `QLabel *topLabel();` ← 返回类型不支持: QLabel *
- `QLabel *bottomLabel();` ← 返回类型不支持: QLabel *

## DColoredProgressBar — 1 方法已生成, 2 跳过
- `void addThreshold(int threshold, QBrush brush);` ← 参数类型不支持: QBrush
- `QList<int> thresholds() const;` ← 返回类型不支持: QList<int>

## DComboBox — 1 方法已生成, 1 跳过
- `virtual bool eventFilter(QObject *watched, QEvent *event) override;` ← 参数类型不支持: QObject

## DCrumbTextFormat — 4 方法已生成, 6 跳过
- `QColor tagColor() const;` ← 返回类型不支持: QColor
- `void setTagColor(const QColor &color);` ← 参数类型不支持: const QColor
- `QColor textColor() const;` ← 返回类型不支持: QColor
- `void setTextColor(const QColor &color);` ← 参数类型不支持: const QColor
- `QBrush background() const;` ← 返回类型不支持: QBrush
- `void setBackground(const QBrush &background);` ← 参数类型不支持: const QBrush

## DCrumbEdit — 11 方法已生成, 6 跳过
- `bool insertCrumb(const DCrumbTextFormat &format, int pos = -1);` ← 参数类型不支持: const DCrumbTextFormat
- `bool appendCrumb(const DCrumbTextFormat &format);` ← 参数类型不支持: const DCrumbTextFormat
- `QStringList crumbList() const;` ← 返回类型不支持: QStringList
- `DCrumbTextFormat crumbTextFormat(const QString &text) const;` ← 返回类型不支持: DCrumbTextFormat
- `DCrumbTextFormat makeTextFormat() const;` ← 返回类型不支持: DCrumbTextFormat
- `DCrumbTextFormat makeTextFormat(CrumbType type) const;` ← 返回类型不支持: DCrumbTextFormat

## DDrawer — 7 方法已生成, 3 跳过
- `void setHeader(QWidget *header);` ← 参数类型不支持: QWidget
- `void setContent(QWidget *content, Qt::Alignment alignment = Qt::AlignHCenter);` ← 参数类型不支持: QWidget
- `void setAnimationEasingCurve(QEasingCurve curve);` ← 参数类型不支持: QEasingCurve

## DDrawerGroup — 3 方法已生成, 5 跳过
- `QList<DDrawer *> expands() const;` ← 返回类型不支持: QList<DDrawer *>
- `void addExpand(DDrawer *expand, int id = -1);` ← 参数类型不支持: DDrawer
- `void setId(DDrawer *expand, int id);` ← 参数类型不支持: DDrawer
- `void removeExpand(DDrawer *expand);` ← 参数类型不支持: DDrawer
- `int id(DDrawer *expand) const;` ← 参数类型不支持: DDrawer

## DFileChooserEdit — 1 方法已生成, 10 跳过
- `void setFileMode(QFileDialog::FileMode mode);` ← 参数类型不支持: QFileDialog::FileMode
- `QFileDialog::FileMode fileMode() const;` ← 返回类型不支持: QFileDialog::FileMode
- `void setNameFilters(const QStringList &filters);` ← 参数类型不支持: const QStringList
- `QStringList nameFilters() const;` ← 返回类型不支持: QStringList
- `void setDirectoryUrl(const QUrl &directory);` ← 参数类型不支持: const QUrl
- `QUrl directoryUrl();` ← 返回类型不支持: QUrl
- `void setDialogDisplayPosition(DialogDisplayPosition dialogDisplayPosition);` ← 参数类型不支持: DialogDisplayPosition
- `DFileChooserEdit::DialogDisplayPosition dialogDisplayPosition() const;` ← 返回类型不支持: DFileChooserEdit::DialogDisplayPosition
- `void setFileDialog(QFileDialog *fileDialog);` ← 参数类型不支持: QFileDialog
- `QFileDialog *fileDialog() const;` ← 返回类型不支持: QFileDialog *

## DFileDialog — 5 方法已生成, 7 跳过
- `explicit DFileDialog(QWidget *parent = Q_NULLPTR,` ← 签名解析失败
- `const QString &caption = QString(),` ← 签名解析失败
- `const QString &directory = QString(),` ← 签名解析失败
- `const QString &filter = QString());` ← 签名解析失败
- `void addComboBox(const QString &text, const QStringList &data);` ← 参数类型不支持: const QStringList
- `void addComboBox(const QString &text, const DComboBoxOptions &options);` ← 参数类型不支持: const DComboBoxOptions
- `void addLineEdit(const QString &text, const DLineEditOptions &options);` ← 参数类型不支持: const DLineEditOptions

## DFontComboBox — 0 方法已生成, 7 跳过
- `void setWritingSystem(QFontDatabase::WritingSystem);` ← 参数类型不支持: QFontDatabase::WritingSystem
- `QFontDatabase::WritingSystem writingSystem() const;` ← 返回类型不支持: QFontDatabase::WritingSystem
- `void setFontFilters(QFontComboBox::FontFilters filters);` ← 参数类型不支持: QFontComboBox::FontFilters
- `QFontComboBox::FontFilters fontFilters() const;` ← 返回类型不支持: QFontComboBox::FontFilters
- `QFont currentFont() const;` ← 返回类型不支持: QFont
- `virtual QSize sizeHint() const override;` ← 返回类型不支持: QSize
- `void setCurrentFont(const QFont &f);` ← 参数类型不支持: const QFont

## DGraphicsGlowEffect — 0 方法已生成, 15 跳过
- `inline void setOffset(qreal dx, qreal dy) {m_xOffset = dx; m_yOffset = dy;}` ← 签名解析失败
- `inline void setXOffset(qreal dx) {m_xOffset = dx;}` ← 签名解析失败
- `inline qreal xOffset() const {return m_xOffset;}` ← 签名解析失败
- `inline void setYOffset(qreal dy) {m_yOffset = dy;}` ← 签名解析失败
- `inline qreal yOffset() const {return m_yOffset;}` ← 签名解析失败
- `inline void setDistance(qreal distance) { m_distance = distance; updateBoundingR` ← 签名解析失败
- `inline qreal distance() const { return m_distance; }` ← 签名解析失败
- `inline void setBlurRadius(qreal blurRadius) { m_blurRadius = blurRadius; updateB` ← 签名解析失败
- `inline qreal blurRadius() const { return m_blurRadius; }` ← 签名解析失败
- `inline void setColor(const QColor &color) { m_color = color; }` ← 签名解析失败
- `inline QColor color() const { return m_color; }` ← 签名解析失败
- `inline qreal opacity() const { return m_opacity; }` ← 签名解析失败
- `inline void setOpacity(qreal opacity) { m_opacity = opacity; }` ← 签名解析失败
- `void draw(QPainter *painter);` ← 参数类型不支持: QPainter
- `QRectF boundingRectFor(const QRectF &rect) const;` ← 返回类型不支持: QRectF

## DHeaderLine — 2 方法已生成, 1 跳过
- `void setContent(QWidget *content);` ← 参数类型不支持: QWidget

## DImageViewer — 18 方法已生成, 5 跳过
- `QImage image() const;` ← 返回类型不支持: QImage
- `void setImage(const QImage &image);` ← 参数类型不支持: const QImage
- `QRect visibleImageRect() const;` ← 返回类型不支持: QRect
- `Q_SLOT void scaleAtPoint(QPoint pos, qreal factor);` ← 返回类型不支持: Q_SLOT void
- `QRect cropImageRect() const;` ← 返回类型不支持: QRect

## DIpv4LineEdit — 8 方法已生成, 1 跳过
- `Qt::Alignment alignment() const;` ← 返回类型不支持: Qt::Alignment

## DKeySequenceEdit — 1 方法已生成, 4 跳过
- `bool setKeySequence(const QKeySequence &keySequence);` ← 参数类型不支持: const QKeySequence
- `QKeySequence keySequence();` ← 返回类型不支持: QKeySequence
- `void ShortcutDirection(Qt::AlignmentFlag alig);` ← 参数类型不支持: Qt::AlignmentFlag
- `QString getKeySequence(QKeySequence sequence);` ← 参数类型不支持: QKeySequence

## DLineEdit — 24 方法已生成, 9 跳过
- `QLineEdit *lineEdit() const;` ← 返回类型不支持: QLineEdit *
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← 参数类型不支持: QWidget
- `void setAlertMessageAlignment(Qt::Alignment alignment);` ← 参数类型不支持: Qt::Alignment
- `Qt::Alignment alertMessageAlignment() const;` ← 返回类型不支持: Qt::Alignment
- `void setLeftWidgets(const QList<QWidget *> &list);` ← 参数类型不支持: const QList<QWidget *>
- `void setRightWidgets(const QList<QWidget *> &list);` ← 参数类型不支持: const QList<QWidget *>
- `QLineEdit::EchoMode echoMode() const;` ← 返回类型不支持: QLineEdit::EchoMode
- `void setEchoMode(QLineEdit::EchoMode mode);` ← 参数类型不支持: QLineEdit::EchoMode
- `void setContextMenuPolicy(Qt::ContextMenuPolicy policy);` ← 参数类型不支持: Qt::ContextMenuPolicy

## DListView — 11 方法已生成, 21 跳过
- `/// return true if rect intersects contentsVisualRect+qMax(cacheBuffer,cacheCoun` ← 签名解析失败
- `State state() const;` ← 返回类型不支持: State
- `bool isActiveRect(const QRect &rect) const;` ← 参数类型不支持: const QRect
- `bool isVisualRect(const QRect &rect) const;` ← 参数类型不支持: const QRect
- `Qt::Orientation orientation() const;` ← 返回类型不支持: Qt::Orientation
- `void setModel(QAbstractItemModel *model);` ← 参数类型不支持: QAbstractItemModel
- `QSize minimumSizeHint() const;` ← 返回类型不支持: QSize
- `DStyledItemDelegate::BackgroundType backgroundType() const;` ← 返回类型不支持: DStyledItemDelegate::BackgroundType
- `QMargins itemMargins() const;` ← 返回类型不支持: QMargins
- `QSize itemSize() const;` ← 返回类型不支持: QSize
- `bool addItem(const QVariant &data);` ← 参数类型不支持: const QVariant
- `bool addItems(const QVariantList &datas);` ← 参数类型不支持: const QVariantList
- `bool insertItem(int index, const QVariant &data);` ← 参数类型不支持: const QVariant
- `bool insertItems(int index, const QVariantList &datas);` ← 参数类型不支持: const QVariantList
- `int addHeaderWidget(QWidget *widget);` ← 参数类型不支持: QWidget
- `int addFooterWidget(QWidget *widget);` ← 参数类型不支持: QWidget
- `void setOrientation(QListView::Flow flow, bool wrapping);` ← 参数类型不支持: QListView::Flow
- `void edit(const QModelIndex &index);` ← 参数类型不支持: const QModelIndex
- `void setBackgroundType(DStyledItemDelegate::BackgroundType backgroundType);` ← 参数类型不支持: DStyledItemDelegate::BackgroundType
- `void setItemMargins(const QMargins &itemMargins);` ← 参数类型不支持: const QMargins
- `void setItemSize(QSize itemSize);` ← 参数类型不支持: QSize

## DLoadingIndicator — 11 方法已生成, 11 跳过
- `QColor backgroundColor() const;` ← 返回类型不支持: QColor
- `QPixmap imageSource() const;` ← 返回类型不支持: QPixmap
- `QEasingCurve::Type aniEasingType() const;` ← 返回类型不支持: QEasingCurve::Type
- `QSize sizeHint() const;` ← 返回类型不支持: QSize
- `RotationDirection direction() const;` ← 返回类型不支持: RotationDirection
- `void setAniEasingCurve(const QEasingCurve & easing);` ← 参数类型不支持: const QEasingCurve &
- `void setBackgroundColor(const QColor &color);` ← 参数类型不支持: const QColor
- `void setRotate(QVariant angle);` ← 参数类型不支持: QVariant
- `void setImageSource(const QPixmap &imageSource);` ← 参数类型不支持: const QPixmap
- `void setAniEasingType(QEasingCurve::Type aniEasingType);` ← 参数类型不支持: QEasingCurve::Type
- `void setDirection(RotationDirection direction);` ← 参数类型不支持: RotationDirection

## DMPRISControl — 2 方法已生成, 1 跳过
- `void setPictureSize(const QSize &size);` ← 参数类型不支持: const QSize

## DPageIndicator — 12 方法已生成, 4 跳过
- `QColor pointColor() const;` ← 返回类型不支持: QColor
- `void setPointColor(QColor color);` ← 参数类型不支持: QColor
- `QColor secondaryPointColor() const;` ← 返回类型不支持: QColor
- `void setSecondaryPointColor(QColor color);` ← 参数类型不支持: QColor

## DPasswordEdit — 3 方法已生成, 1 跳过
- `void setEchoMode(QLineEdit::EchoMode mode);` ← 参数类型不支持: QLineEdit::EchoMode

## DPictureSequenceView — 7 方法已生成, 3 跳过
- `void setPictureSequence(const QString &srcFormat, const QPair<int, int> &range, ` ← 参数类型不支持: const QPair<int, int>
- `void setPictureSequence(const QStringList &sequence, const bool autoScale = fals` ← 参数类型不支持: const QStringList
- `void setPictureSequence(const QList<QPixmap> &sequence, const bool autoScale = f` ← 参数类型不支持: const QList<QPixmap>

## DPrintPreviewWidget — 37 方法已生成, 16 跳过
- `void setPageRange(const QVector<int> &rangePages);` ← 参数类型不支持: const QVector<int>
- `void setPageRangeMode(PageRange mode);` ← 参数类型不支持: PageRange
- `PageRange pageRangeMode();` ← 返回类型不支持: PageRange
- `void setColorMode(const DPrinter::ColorMode &colorMode);` ← 参数类型不支持: const DPrinter::ColorMode
- `void setOrientation(const DPrinter::Orientation &pageOrientation);` ← 参数类型不支持: const DPrinter::Orientation
- `DPrinter::ColorMode getColorMode();` ← 返回类型不支持: DPrinter::ColorMode
- `void setWaterMargImage(const QImage &image);` ← 参数类型不支持: const QImage
- `void setWaterMarkFont(const QFont &font);` ← 参数类型不支持: const QFont
- `QColor waterMarkColor() const;` ← 返回类型不支持: QColor
- `void setWaterMarkColor(const QColor &color);` ← 参数类型不支持: const QColor
- `void setImposition(Imposition im);` ← 参数类型不支持: Imposition
- `Imposition imposition() const;` ← 返回类型不支持: Imposition
- `void setOrder(Order order);` ← 参数类型不支持: Order
- `DPrintPreviewWidget::Order order() const;` ← 返回类型不支持: DPrintPreviewWidget::Order
- `void setPrintMode(PrintMode pt);` ← 参数类型不支持: PrintMode
- `void themeTypeChanged(DGuiApplicationHelper::ColorType themeType);` ← 参数类型不支持: DGuiApplicationHelper::ColorType

## DSearchComboBox — 1 方法已生成, 0 跳过

## DSearchEdit — 7 方法已生成, 0 跳过

## DSettingsDialog — 5 方法已生成, 3 跳过
- `void setIcon(const QIcon &icon);` ← 参数类型不支持: const QIcon
- `void updateSettings(DTK_CORE_NAMESPACE::DSettings *settings);` ← 参数类型不支持: DTK_CORE_NAMESPACE::DSettings
- `void updateSettings(const QByteArray &translateContext, DTK_CORE_NAMESPACE::DSet` ← 参数类型不支持: DTK_CORE_NAMESPACE::DSettings

## DSettingsWidgetFactory — 0 方法已生成, 7 跳过
- `QPair<QWidget*, QWidget*> createItem(QPointer<DTK_CORE_NAMESPACE::DSettingsOptio` ← 签名解析失败
- `QPair<QWidget*, QWidget*> createItem(const QByteArray &translateContext, QPointe` ← 签名解析失败
- `static QPair<QWidget*, QWidget*> createStandardItem(const QByteArray &translateC` ← 签名解析失败
- `void registerWidget(const QString &viewType, std::function<WidgetCreateHandler> ` ← 参数类型不支持: std::function<WidgetCreateHandler>
- `void registerWidget(const QString &viewType, std::function<ItemCreateHandler> ha` ← 参数类型不支持: std::function<ItemCreateHandler>
- `QWidget *createWidget(QPointer<DTK_CORE_NAMESPACE::DSettingsOption> option);` ← 参数类型不支持: QPointer<DTK_CORE_NAMESPACE::DSettingsOption>
- `QWidget *createWidget(const QByteArray &translateContext, QPointer<DTK_CORE_NAME` ← 参数类型不支持: QPointer<DTK_CORE_NAMESPACE::DSettingsOption>

## DShadowLine — 0 方法已生成, 1 跳过
- `QSize sizeHint() const;` ← 返回类型不支持: QSize

## DSimpleListItem — 0 方法已生成, 3 跳过
- `virtual bool sameAs(DSimpleListItem *item)=0;` ← 参数类型不支持: DSimpleListItem
- `virtual void drawBackground(QRect rect, QPainter *painter, int index, bool isSel` ← 参数类型不支持: QRect
- `virtual void drawForeground(QRect rect, QPainter *painter, int column, int index` ← 参数类型不支持: QRect

## DSimpleListView — 25 方法已生成, 10 跳过
- `* \algorithms a list of SortAlgorithm, SortAlgorithm is function pointer, it's t` ← 签名解析失败
- `* \algorithm the search algorithm, it's type is: 'bool (*) (const DSimpleListIte` ← 签名解析失败
- `void setColumnTitleInfo(QList<QString> titles, QList<int> widths, int height);` ← 参数类型不支持: QList<QString>
- `void setColumnHideFlags(QList<bool> toggleHideFlags, int alwaysVisibleColumn=-1)` ← 参数类型不支持: QList<bool>
- `void setColumnSortingAlgorithms(QList<SortAlgorithm> *algorithms, int sortColumn` ← 参数类型不支持: QList<SortAlgorithm>
- `void setSearchAlgorithm(SearchAlgorithm algorithm);` ← 参数类型不支持: SearchAlgorithm
- `void addItems(QList<DSimpleListItem*> items);` ← 参数类型不支持: QList<DSimpleListItem*>
- `void addSelections(QList<DSimpleListItem*> items, bool recordLastSelection=true)` ← 参数类型不支持: QList<DSimpleListItem*>
- `QList<DSimpleListItem*> getSelections();` ← 返回类型不支持: QList<DSimpleListItem*>
- `void refreshItems(QList<DSimpleListItem*> items);` ← 参数类型不支持: QList<DSimpleListItem*>

## DSizeModeHelper — 0 方法已生成, 2 跳过
- `return DGUI_NAMESPACE::DGuiApplicationHelper::isCompactMode() ? t1 : t2;` ← 签名解析失败
- `static inline T element(const T &t1, const T &t2)` ← 返回类型不支持: inline T

## DSlider — 13 方法已生成, 12 跳过
- `Qt::Orientation orientation() const;` ← 返回类型不支持: Qt::Orientation
- `QSlider *slider();` ← 返回类型不支持: QSlider *
- `void setLeftIcon(const QIcon &left);` ← 参数类型不支持: const QIcon
- `void setRightIcon(const QIcon &right);` ← 参数类型不支持: const QIcon
- `void setIconSize(const QSize &size);` ← 参数类型不支持: const QSize
- `void setLeftTicks(const QStringList &info);` ← 参数类型不支持: const QStringList
- `void setRightTicks(const QStringList &info);` ← 参数类型不支持: const QStringList
- `void setAboveTicks(const QStringList &info);` ← 参数类型不支持: const QStringList
- `void setBelowTicks(const QStringList &info);` ← 参数类型不支持: const QStringList
- `void setMarkPositions(QList<int> list);` ← 参数类型不支持: QList<int>
- `QSlider::TickPosition tickPosition() const;` ← 返回类型不支持: QSlider::TickPosition
- `QSize sizeHint() const override;` ← 返回类型不支持: QSize

## DSpinBox — 4 方法已生成, 2 跳过
- `QLineEdit *lineEdit() const;` ← 返回类型不支持: QLineEdit *
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← 参数类型不支持: QWidget

## DDoubleSpinBox — 4 方法已生成, 2 跳过
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← 参数类型不支持: QWidget
- `QLineEdit *lineEdit() const;` ← 返回类型不支持: QLineEdit *

## DSpinner — 3 方法已生成, 1 跳过
- `void setBackgroundColor(QColor color);` ← 参数类型不支持: QColor

## DSwitchButton — 0 方法已生成, 1 跳过
- `QSize sizeHint() const;` ← 返回类型不支持: QSize

## DSwitchLineExpand — 3 方法已生成, 0 跳过

## DTabletWindowOptionButton — 0 方法已生成, 1 跳过
- `QSize sizeHint() const override;` ← 返回类型不支持: QSize

## DTickEffect — 5 方法已生成, 1 跳过
- `void setDirection(Direction direction);` ← 参数类型不支持: Direction

## DTipLabel — 0 方法已生成, 2 跳过
- `void show(const QPoint &pos);` ← 参数类型不支持: const QPoint
- `void setForegroundRole(DPalette::ColorType color);` ← 参数类型不支持: DPalette::ColorType

## DTitlebarToolBaseInterface — 3 方法已生成, 2 跳过
- `explicit DTitlebarToolBaseInterface(QObject *parent = nullptr) : QObject(parent)` ← 签名解析失败
- `virtual ~DTitlebarToolBaseInterface(){}` ← 签名解析失败

## DTitleBarToolInterface — 1 方法已生成, 2 跳过
- `explicit DTitleBarToolInterface(QObject *parent = nullptr) : DTitlebarToolBaseIn` ← 签名解析失败
- `virtual ~DTitleBarToolInterface(){}` ← 签名解析失败

## DTitleBarSpacerInterface — 2 方法已生成, 2 跳过
- `explicit DTitleBarSpacerInterface(QObject *parent = nullptr) : DTitlebarToolBase` ← 签名解析失败
- `virtual ~DTitleBarSpacerInterface(){}` ← 签名解析失败

## DTitlebarSettings — 1 方法已生成, 1 跳过
- `bool initilize(QList<DTitlebarToolBaseInterface *> &tools, const QString &path);` ← 参数类型不支持: QList<DTitlebarToolBaseInterface *>

## DToolButton — 0 方法已生成, 2 跳过
- `void setAlignment(Qt::Alignment flag);` ← 参数类型不支持: Qt::Alignment
- `Qt::Alignment alignment() const;` ← 返回类型不支持: Qt::Alignment

## DWaterProgress — 5 方法已生成, 0 跳过

## DWindowCloseButton — 0 方法已生成, 1 跳过
- `QSize sizeHint() const override;` ← 返回类型不支持: QSize

## DWindowMaxButton — 2 方法已生成, 1 跳过
- `QSize sizeHint() const override;` ← 返回类型不支持: QSize

## DWindowMinButton — 0 方法已生成, 1 跳过
- `QSize sizeHint() const override;` ← 返回类型不支持: QSize

## DWindowOptionButton — 0 方法已生成, 1 跳过
- `QSize sizeHint() const override;` ← 返回类型不支持: QSize

## DWindowQuitFullButton — 0 方法已生成, 1 跳过
- `QSize sizeHint() const override;` ← 返回类型不支持: QSize
