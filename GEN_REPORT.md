# DTK6 widget binding 覆盖报告

类: 60, 已生成方法: 416, 跳过: 156

## DAbstractDialog — 8 方法已生成, 2 跳过
- `inline void move(int x, int y)` ← 返回类型不支持: inline void
- `inline void setGeometry(int x, int y, int width, int height)` ← 返回类型不支持: inline void

## DAccessibilityChecker — 4 方法已生成, 0 跳过

## DAlertControl — 9 方法已生成, 1 跳过
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← 参数类型不支持: QWidget

## DArrowButton — 3 方法已生成, 0 跳过

## DArrowLineDrawer — 2 方法已生成, 0 跳过

## DArrowRectangle — 38 方法已生成, 1 跳过
- `void setContent(QWidget *content);` ← 参数类型不支持: QWidget

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

## DBlurEffectWidget — 19 方法已生成, 3 跳过
- `void setMaskPath(const QPainterPath &path);` ← 参数类型不支持: const QPainterPath
- `void setSourceImage(const QImage &image, bool autoScale = true);` ← 参数类型不支持: const QImage
- `void updateBlurSourceImage(const QRegion &ren);` ← 参数类型不支持: const QRegion

## DCircleProgress — 10 方法已生成, 2 跳过
- `QLabel *topLabel();` ← 返回类型不支持: QLabel *
- `QLabel *bottomLabel();` ← 返回类型不支持: QLabel *

## DColoredProgressBar — 1 方法已生成, 2 跳过
- `void addThreshold(int threshold, QBrush brush);` ← 参数类型不支持: QBrush
- `QList<int> thresholds() const;` ← 返回类型不支持: QList<int>

## DComboBox — 1 方法已生成, 1 跳过
- `virtual bool eventFilter(QObject *watched, QEvent *event) override;` ← 参数类型不支持: QObject

## DCrumbTextFormat — 8 方法已生成, 2 跳过
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

## DFileChooserEdit — 3 方法已生成, 8 跳过
- `void setFileMode(QFileDialog::FileMode mode);` ← 参数类型不支持: QFileDialog::FileMode
- `QFileDialog::FileMode fileMode() const;` ← 返回类型不支持: QFileDialog::FileMode
- `void setNameFilters(const QStringList &filters);` ← 参数类型不支持: const QStringList
- `QStringList nameFilters() const;` ← 返回类型不支持: QStringList
- `void setDirectoryUrl(const QUrl &directory);` ← 参数类型不支持: const QUrl
- `QUrl directoryUrl();` ← 返回类型不支持: QUrl
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

## DFontComboBox — 3 方法已生成, 4 跳过
- `void setWritingSystem(QFontDatabase::WritingSystem);` ← 参数类型不支持: QFontDatabase::WritingSystem
- `QFontDatabase::WritingSystem writingSystem() const;` ← 返回类型不支持: QFontDatabase::WritingSystem
- `void setFontFilters(QFontComboBox::FontFilters filters);` ← 参数类型不支持: QFontComboBox::FontFilters
- `QFontComboBox::FontFilters fontFilters() const;` ← 返回类型不支持: QFontComboBox::FontFilters

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

## DImageViewer — 20 方法已生成, 3 跳过
- `QImage image() const;` ← 返回类型不支持: QImage
- `void setImage(const QImage &image);` ← 参数类型不支持: const QImage
- `Q_SLOT void scaleAtPoint(QPoint pos, qreal factor);` ← 返回类型不支持: Q_SLOT void

## DIpv4LineEdit — 9 方法已生成, 0 跳过

## DKeySequenceEdit — 2 方法已生成, 3 跳过
- `bool setKeySequence(const QKeySequence &keySequence);` ← 参数类型不支持: const QKeySequence
- `QKeySequence keySequence();` ← 返回类型不支持: QKeySequence
- `QString getKeySequence(QKeySequence sequence);` ← 参数类型不支持: QKeySequence

## DLineEdit — 27 方法已生成, 6 跳过
- `QLineEdit *lineEdit() const;` ← 返回类型不支持: QLineEdit *
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← 参数类型不支持: QWidget
- `void setLeftWidgets(const QList<QWidget *> &list);` ← 参数类型不支持: const QList<QWidget *>
- `void setRightWidgets(const QList<QWidget *> &list);` ← 参数类型不支持: const QList<QWidget *>
- `QLineEdit::EchoMode echoMode() const;` ← 返回类型不支持: QLineEdit::EchoMode
- `void setEchoMode(QLineEdit::EchoMode mode);` ← 参数类型不支持: QLineEdit::EchoMode

## DListView — 17 方法已生成, 15 跳过
- `/// return true if rect intersects contentsVisualRect+qMax(cacheBuffer,cacheCoun` ← 签名解析失败
- `State state() const;` ← 返回类型不支持: State
- `void setModel(QAbstractItemModel *model);` ← 参数类型不支持: QAbstractItemModel
- `DStyledItemDelegate::BackgroundType backgroundType() const;` ← 返回类型不支持: DStyledItemDelegate::BackgroundType
- `QMargins itemMargins() const;` ← 返回类型不支持: QMargins
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

## DLoadingIndicator — 18 方法已生成, 4 跳过
- `QEasingCurve::Type aniEasingType() const;` ← 返回类型不支持: QEasingCurve::Type
- `void setAniEasingCurve(const QEasingCurve & easing);` ← 参数类型不支持: const QEasingCurve &
- `void setRotate(QVariant angle);` ← 参数类型不支持: QVariant
- `void setAniEasingType(QEasingCurve::Type aniEasingType);` ← 参数类型不支持: QEasingCurve::Type

## DMPRISControl — 3 方法已生成, 0 跳过

## DPageIndicator — 16 方法已生成, 0 跳过

## DPasswordEdit — 3 方法已生成, 1 跳过
- `void setEchoMode(QLineEdit::EchoMode mode);` ← 参数类型不支持: QLineEdit::EchoMode

## DPictureSequenceView — 7 方法已生成, 3 跳过
- `void setPictureSequence(const QString &srcFormat, const QPair<int, int> &range, ` ← 参数类型不支持: const QPair<int, int>
- `void setPictureSequence(const QStringList &sequence, const bool autoScale = fals` ← 参数类型不支持: const QStringList
- `void setPictureSequence(const QList<QPixmap> &sequence, const bool autoScale = f` ← 参数类型不支持: const QList<QPixmap>

## DPrintPreviewWidget — 47 方法已生成, 6 跳过
- `void setPageRange(const QVector<int> &rangePages);` ← 参数类型不支持: const QVector<int>
- `void setColorMode(const DPrinter::ColorMode &colorMode);` ← 参数类型不支持: const DPrinter::ColorMode
- `void setOrientation(const DPrinter::Orientation &pageOrientation);` ← 参数类型不支持: const DPrinter::Orientation
- `DPrinter::ColorMode getColorMode();` ← 返回类型不支持: DPrinter::ColorMode
- `void setWaterMargImage(const QImage &image);` ← 参数类型不支持: const QImage
- `void themeTypeChanged(DGuiApplicationHelper::ColorType themeType);` ← 参数类型不支持: DGuiApplicationHelper::ColorType

## DSearchComboBox — 1 方法已生成, 0 跳过

## DSearchEdit — 7 方法已生成, 0 跳过

## DSettingsDialog — 6 方法已生成, 2 跳过
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

## DShadowLine — 1 方法已生成, 0 跳过

## DSimpleListItem — 0 方法已生成, 3 跳过
- `virtual bool sameAs(DSimpleListItem *item)=0;` ← 参数类型不支持: DSimpleListItem
- `virtual void drawBackground(QRect rect, QPainter *painter, int index, bool isSel` ← 参数类型不支持: QPainter
- `virtual void drawForeground(QRect rect, QPainter *painter, int column, int index` ← 参数类型不支持: QPainter

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

## DSlider — 18 方法已生成, 7 跳过
- `QSlider *slider();` ← 返回类型不支持: QSlider *
- `void setLeftTicks(const QStringList &info);` ← 参数类型不支持: const QStringList
- `void setRightTicks(const QStringList &info);` ← 参数类型不支持: const QStringList
- `void setAboveTicks(const QStringList &info);` ← 参数类型不支持: const QStringList
- `void setBelowTicks(const QStringList &info);` ← 参数类型不支持: const QStringList
- `void setMarkPositions(QList<int> list);` ← 参数类型不支持: QList<int>
- `QSlider::TickPosition tickPosition() const;` ← 返回类型不支持: QSlider::TickPosition

## DSpinBox — 4 方法已生成, 2 跳过
- `QLineEdit *lineEdit() const;` ← 返回类型不支持: QLineEdit *
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← 参数类型不支持: QWidget

## DDoubleSpinBox — 4 方法已生成, 2 跳过
- `void showAlertMessage(const QString &text, QWidget *follower, int duration = 300` ← 参数类型不支持: QWidget
- `QLineEdit *lineEdit() const;` ← 返回类型不支持: QLineEdit *

## DSpinner — 4 方法已生成, 0 跳过

## DSwitchButton — 1 方法已生成, 0 跳过

## DSwitchLineExpand — 3 方法已生成, 0 跳过

## DTabletWindowOptionButton — 1 方法已生成, 0 跳过

## DTickEffect — 6 方法已生成, 0 跳过

## DTipLabel — 1 方法已生成, 1 跳过
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

## DToolButton — 2 方法已生成, 0 跳过

## DWaterProgress — 5 方法已生成, 0 跳过

## DWindowCloseButton — 1 方法已生成, 0 跳过

## DWindowMaxButton — 3 方法已生成, 0 跳过

## DWindowMinButton — 1 方法已生成, 0 跳过

## DWindowOptionButton — 1 方法已生成, 0 跳过

## DWindowQuitFullButton — 1 方法已生成, 0 跳过
