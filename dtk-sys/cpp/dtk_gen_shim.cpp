// 自动生成 by tools/gen.py，勿手改
#include "dtk_gen_shim.h"

namespace dtkrs {
DAbstractDialog *gen_d_abstract_dialog_new() { return new DAbstractDialog; }
int32_t gen_d_abstract_dialog_display_position(DAbstractDialog *self) { return static_cast<int32_t>(self->displayPosition()); }
void gen_d_abstract_dialog_move_(DAbstractDialog *self, QPoint * pos) { self->move(*pos); }
void gen_d_abstract_dialog_set_geometry(DAbstractDialog *self, QRect * rect) { self->setGeometry(*rect); }
void gen_d_abstract_dialog_move_to_center(DAbstractDialog *self) { self->moveToCenter(); }
void gen_d_abstract_dialog_move_to_top_right(DAbstractDialog *self) { self->moveToTopRight(); }
void gen_d_abstract_dialog_move_to_center_by_rect(DAbstractDialog *self, QRect * rect) { self->moveToCenterByRect(*rect); }
void gen_d_abstract_dialog_move_to_top_right_by_rect(DAbstractDialog *self, QRect * rect) { self->moveToTopRightByRect(*rect); }
void gen_d_abstract_dialog_set_display_position(DAbstractDialog *self, int32_t displayPosition) { self->setDisplayPosition(static_cast<DAbstractDialog::DisplayPosition>(displayPosition)); }
DAccessibilityChecker *gen_d_accessibility_checker_new() { return new DAccessibilityChecker; }
void gen_d_accessibility_checker_set_output_format(DAccessibilityChecker *self, int32_t format) { self->setOutputFormat(static_cast<DAccessibilityChecker::OutputFormat>(format)); }
int32_t gen_d_accessibility_checker_output_format(DAccessibilityChecker *self) { return static_cast<int32_t>(self->outputFormat()); }
bool gen_d_accessibility_checker_check(DAccessibilityChecker *self) { return self->check(); }
void gen_d_accessibility_checker_start(DAccessibilityChecker *self, int32_t msec) { self->start(msec); }
void gen_d_alert_control_set_alert(DAlertControl *self, bool isAlert) { self->setAlert(isAlert); }
bool gen_d_alert_control_is_alert(DAlertControl *self) { return self->isAlert(); }
void gen_d_alert_control_set_alert_color(DAlertControl *self, QColor * c) { self->setAlertColor(*c); }
QColor * gen_d_alert_control_alert_color(DAlertControl *self) { return new QColor(self->alertColor()); }
QColor * gen_d_alert_control_default_alert_color(DAlertControl *self) { return new QColor(self->defaultAlertColor()); }
void gen_d_alert_control_set_message_alignment(DAlertControl *self, int32_t alignment) { self->setMessageAlignment(Qt::Alignment::fromInt(alignment)); }
int32_t gen_d_alert_control_message_alignment(DAlertControl *self) { return (self->messageAlignment()).toInt(); }
void gen_d_alert_control_show_alert_message(DAlertControl *self, rust::Str text, int32_t duration) { self->showAlertMessage(from_rust_str(text), duration); }
void gen_d_alert_control_hide_alert_message(DAlertControl *self) { self->hideAlertMessage(); }
DArrowButton *gen_d_arrow_button_new() { return new DArrowButton; }
void gen_d_arrow_button_set_arrow_direction(DArrowButton *self, int32_t direction) { self->setArrowDirection(static_cast<DArrowButton::ArrowDirection>(direction)); }
int32_t gen_d_arrow_button_arrow_direction(DArrowButton *self) { return self->arrowDirection(); }
int32_t gen_d_arrow_button_button_state(DArrowButton *self) { return self->buttonState(); }
DArrowLineDrawer *gen_d_arrow_line_drawer_new() { return new DArrowLineDrawer; }
void gen_d_arrow_line_drawer_set_title(DArrowLineDrawer *self, rust::Str title) { self->setTitle(from_rust_str(title)); }
void gen_d_arrow_line_drawer_set_expand(DArrowLineDrawer *self, bool value) { self->setExpand(value); }
int32_t gen_d_arrow_rectangle_radius(DArrowRectangle *self) { return self->radius(); }
bool gen_d_arrow_rectangle_radius_force_enabled(DArrowRectangle *self) { return self->radiusForceEnabled(); }
int32_t gen_d_arrow_rectangle_arrow_height(DArrowRectangle *self) { return self->arrowHeight(); }
int32_t gen_d_arrow_rectangle_arrow_width(DArrowRectangle *self) { return self->arrowWidth(); }
int32_t gen_d_arrow_rectangle_arrow_x(DArrowRectangle *self) { return self->arrowX(); }
int32_t gen_d_arrow_rectangle_arrow_y(DArrowRectangle *self) { return self->arrowY(); }
int32_t gen_d_arrow_rectangle_margin(DArrowRectangle *self) { return self->margin(); }
int32_t gen_d_arrow_rectangle_border_width(DArrowRectangle *self) { return self->borderWidth(); }
QColor * gen_d_arrow_rectangle_border_color(DArrowRectangle *self) { return new QColor(self->borderColor()); }
QColor * gen_d_arrow_rectangle_background_color(DArrowRectangle *self) { return new QColor(self->backgroundColor()); }
int32_t gen_d_arrow_rectangle_arrow_direction(DArrowRectangle *self) { return static_cast<int32_t>(self->arrowDirection()); }
void gen_d_arrow_rectangle_set_radius(DArrowRectangle *self, int32_t value) { self->setRadius(value); }
void gen_d_arrow_rectangle_set_radius_force_enable(DArrowRectangle *self, bool enable) { self->setRadiusForceEnable(enable); }
void gen_d_arrow_rectangle_set_arrow_height(DArrowRectangle *self, int32_t value) { self->setArrowHeight(value); }
void gen_d_arrow_rectangle_set_arrow_width(DArrowRectangle *self, int32_t value) { self->setArrowWidth(value); }
void gen_d_arrow_rectangle_set_arrow_x(DArrowRectangle *self, int32_t value) { self->setArrowX(value); }
void gen_d_arrow_rectangle_set_arrow_y(DArrowRectangle *self, int32_t value) { self->setArrowY(value); }
void gen_d_arrow_rectangle_set_margin(DArrowRectangle *self, int32_t value) { self->setMargin(value); }
void gen_d_arrow_rectangle_set_border_width(DArrowRectangle *self, int32_t borderWidth) { self->setBorderWidth(borderWidth); }
void gen_d_arrow_rectangle_set_border_color(DArrowRectangle *self, QColor * borderColor) { self->setBorderColor(*borderColor); }
void gen_d_arrow_rectangle_set_background_color(DArrowRectangle *self, QColor * backgroundColor) { self->setBackgroundColor(*backgroundColor); }
void gen_d_arrow_rectangle_set_background_color_2(DArrowRectangle *self, int32_t type_) { self->setBackgroundColor(static_cast<DBlurEffectWidget::MaskColorType>(type_)); }
void gen_d_arrow_rectangle_set_arrow_direction(DArrowRectangle *self, int32_t value) { self->setArrowDirection(static_cast<DArrowRectangle::ArrowDirection>(value)); }
void gen_d_arrow_rectangle_set_width(DArrowRectangle *self, int32_t value) { self->setWidth(value); }
void gen_d_arrow_rectangle_set_height(DArrowRectangle *self, int32_t value) { self->setHeight(value); }
void gen_d_arrow_rectangle_show(DArrowRectangle *self, int32_t x, int32_t y) { self->show(x, y); }
QWidget * gen_d_arrow_rectangle_get_content(DArrowRectangle *self) { return self->getContent(); }
void gen_d_arrow_rectangle_resize_with_content(DArrowRectangle *self) { self->resizeWithContent(); }
void gen_d_arrow_rectangle_move_(DArrowRectangle *self, int32_t x, int32_t y) { self->move(x, y); }
QSize * gen_d_arrow_rectangle_get_fixed_size(DArrowRectangle *self) { return new QSize(self->getFixedSize()); }
double gen_d_arrow_rectangle_shadow_x_offset(DArrowRectangle *self) { return self->shadowXOffset(); }
double gen_d_arrow_rectangle_shadow_y_offset(DArrowRectangle *self) { return self->shadowYOffset(); }
double gen_d_arrow_rectangle_shadow_blur_radius(DArrowRectangle *self) { return self->shadowBlurRadius(); }
void gen_d_arrow_rectangle_set_shadow_blur_radius(DArrowRectangle *self, double shadowBlurRadius) { self->setShadowBlurRadius(shadowBlurRadius); }
void gen_d_arrow_rectangle_set_shadow_x_offset(DArrowRectangle *self, double shadowXOffset) { self->setShadowXOffset(shadowXOffset); }
void gen_d_arrow_rectangle_set_shadow_y_offset(DArrowRectangle *self, double shadowYOffset) { self->setShadowYOffset(shadowYOffset); }
void gen_d_arrow_rectangle_set_left_right_radius(DArrowRectangle *self, bool enable) { self->setLeftRightRadius(enable); }
void gen_d_arrow_rectangle_set_radius_arrow_style_enable(DArrowRectangle *self, bool enable) { self->setRadiusArrowStyleEnable(enable); }
DBackgroundGroup *gen_d_background_group_new() { return new DBackgroundGroup; }
bool gen_d_background_group_use_widget_background(DBackgroundGroup *self) { return self->useWidgetBackground(); }
void gen_d_background_group_set_item_spacing(DBackgroundGroup *self, int32_t spacing) { self->setItemSpacing(spacing); }
void gen_d_background_group_set_use_widget_background(DBackgroundGroup *self, bool useWidgetBackground) { self->setUseWidgetBackground(useWidgetBackground); }
DBaseLine *gen_d_base_line_new() { return new DBaseLine; }
void gen_d_base_line_set_left_margin(DBaseLine *self, int32_t margin) { self->setLeftMargin(margin); }
void gen_d_base_line_set_right_margin(DBaseLine *self, int32_t margin) { self->setRightMargin(margin); }
int32_t gen_d_base_line_left_margin(DBaseLine *self) { return self->leftMargin(); }
int32_t gen_d_base_line_right_margin(DBaseLine *self) { return self->rightMargin(); }
DBlurEffectWidget *gen_d_blur_effect_widget_new() { return new DBlurEffectWidget; }
int32_t gen_d_blur_effect_widget_radius(DBlurEffectWidget *self) { return self->radius(); }
int32_t gen_d_blur_effect_widget_mode(DBlurEffectWidget *self) { return static_cast<int32_t>(self->mode()); }
int32_t gen_d_blur_effect_widget_blend_mode(DBlurEffectWidget *self) { return static_cast<int32_t>(self->blendMode()); }
int32_t gen_d_blur_effect_widget_blur_rect_x_radius(DBlurEffectWidget *self) { return self->blurRectXRadius(); }
int32_t gen_d_blur_effect_widget_blur_rect_y_radius(DBlurEffectWidget *self) { return self->blurRectYRadius(); }
bool gen_d_blur_effect_widget_is_full(DBlurEffectWidget *self) { return self->isFull(); }
bool gen_d_blur_effect_widget_blur_enabled(DBlurEffectWidget *self) { return self->blurEnabled(); }
QColor * gen_d_blur_effect_widget_mask_color(DBlurEffectWidget *self) { return new QColor(self->maskColor()); }
uint8_t gen_d_blur_effect_widget_mask_alpha(DBlurEffectWidget *self) { return self->maskAlpha(); }
void gen_d_blur_effect_widget_set_radius(DBlurEffectWidget *self, int32_t radius) { self->setRadius(radius); }
void gen_d_blur_effect_widget_set_mode(DBlurEffectWidget *self, int32_t mode) { self->setMode(static_cast<DBlurEffectWidget::BlurMode>(mode)); }
void gen_d_blur_effect_widget_set_blend_mode(DBlurEffectWidget *self, int32_t blendMode) { self->setBlendMode(static_cast<DBlurEffectWidget::BlendMode>(blendMode)); }
void gen_d_blur_effect_widget_set_blur_rect_x_radius(DBlurEffectWidget *self, int32_t blurRectXRadius) { self->setBlurRectXRadius(blurRectXRadius); }
void gen_d_blur_effect_widget_set_blur_rect_y_radius(DBlurEffectWidget *self, int32_t blurRectYRadius) { self->setBlurRectYRadius(blurRectYRadius); }
void gen_d_blur_effect_widget_set_mask_alpha(DBlurEffectWidget *self, uint8_t alpha) { self->setMaskAlpha(alpha); }
void gen_d_blur_effect_widget_set_mask_color(DBlurEffectWidget *self, QColor * maskColor) { self->setMaskColor(*maskColor); }
void gen_d_blur_effect_widget_set_mask_color_2(DBlurEffectWidget *self, int32_t type_) { self->setMaskColor(static_cast<DBlurEffectWidget::MaskColorType>(type_)); }
void gen_d_blur_effect_widget_set_full(DBlurEffectWidget *self, bool full) { self->setFull(full); }
void gen_d_blur_effect_widget_set_blur_enabled(DBlurEffectWidget *self, bool blurEnabled) { self->setBlurEnabled(blurEnabled); }
DCircleProgress *gen_d_circle_progress_new() { return new DCircleProgress; }
int32_t gen_d_circle_progress_value(DCircleProgress *self) { return self->value(); }
void gen_d_circle_progress_set_value(DCircleProgress *self, int32_t value) { self->setValue(value); }
rust::String gen_d_circle_progress_text(DCircleProgress *self) { return to_rust_string(self->text()); }
void gen_d_circle_progress_set_text(DCircleProgress *self, rust::Str text) { self->setText(from_rust_str(text)); }
QColor * gen_d_circle_progress_background_color(DCircleProgress *self) { return new QColor(self->backgroundColor()); }
void gen_d_circle_progress_set_background_color(DCircleProgress *self, QColor * color) { self->setBackgroundColor(*color); }
QColor * gen_d_circle_progress_chunk_color(DCircleProgress *self) { return new QColor(self->chunkColor()); }
void gen_d_circle_progress_set_chunk_color(DCircleProgress *self, QColor * color) { self->setChunkColor(*color); }
int32_t gen_d_circle_progress_line_width(DCircleProgress *self) { return self->lineWidth(); }
void gen_d_circle_progress_set_line_width(DCircleProgress *self, int32_t width) { self->setLineWidth(width); }
DColoredProgressBar *gen_d_colored_progress_bar_new() { return new DColoredProgressBar; }
void gen_d_colored_progress_bar_remove_threshold(DColoredProgressBar *self, int32_t threshold) { self->removeThreshold(threshold); }
DComboBox *gen_d_combo_box_new() { return new DComboBox; }
void gen_d_combo_box_show_popup(DComboBox *self) { self->showPopup(); }
DCrumbTextFormat *gen_d_crumb_text_format_new() { return new DCrumbTextFormat; }
QColor * gen_d_crumb_text_format_tag_color(DCrumbTextFormat *self) { return new QColor(self->tagColor()); }
void gen_d_crumb_text_format_set_tag_color(DCrumbTextFormat *self, QColor * color) { self->setTagColor(*color); }
rust::String gen_d_crumb_text_format_text(DCrumbTextFormat *self) { return to_rust_string(self->text()); }
void gen_d_crumb_text_format_set_text(DCrumbTextFormat *self, rust::Str text) { self->setText(from_rust_str(text)); }
QColor * gen_d_crumb_text_format_text_color(DCrumbTextFormat *self) { return new QColor(self->textColor()); }
void gen_d_crumb_text_format_set_text_color(DCrumbTextFormat *self, QColor * color) { self->setTextColor(*color); }
int32_t gen_d_crumb_text_format_background_radius(DCrumbTextFormat *self) { return self->backgroundRadius(); }
void gen_d_crumb_text_format_set_background_radius(DCrumbTextFormat *self, int32_t radius) { self->setBackgroundRadius(radius); }
DCrumbEdit *gen_d_crumb_edit_new() { return new DCrumbEdit; }
bool gen_d_crumb_edit_insert_crumb(DCrumbEdit *self, rust::Str text, int32_t pos) { return self->insertCrumb(from_rust_str(text), pos); }
bool gen_d_crumb_edit_append_crumb(DCrumbEdit *self, rust::Str text) { return self->appendCrumb(from_rust_str(text)); }
bool gen_d_crumb_edit_contain_crumb(DCrumbEdit *self, rust::Str text) { return self->containCrumb(from_rust_str(text)); }
bool gen_d_crumb_edit_dual_click_make_crumb(DCrumbEdit *self) { return self->dualClickMakeCrumb(); }
bool gen_d_crumb_edit_crumb_read_only(DCrumbEdit *self) { return self->crumbReadOnly(); }
int32_t gen_d_crumb_edit_crumb_radius(DCrumbEdit *self) { return self->crumbRadius(); }
rust::String gen_d_crumb_edit_splitter(DCrumbEdit *self) { return to_rust_string(self->splitter()); }
void gen_d_crumb_edit_set_crumb_read_only(DCrumbEdit *self, bool crumbReadOnly) { self->setCrumbReadOnly(crumbReadOnly); }
void gen_d_crumb_edit_set_crumb_radius(DCrumbEdit *self, int32_t crumbRadius) { self->setCrumbRadius(crumbRadius); }
void gen_d_crumb_edit_set_splitter(DCrumbEdit *self, rust::Str splitter) { self->setSplitter(from_rust_str(splitter)); }
void gen_d_crumb_edit_set_dual_click_make_crumb(DCrumbEdit *self, bool flag) { self->setDualClickMakeCrumb(flag); }
DDrawer *gen_d_drawer_new() { return new DDrawer; }
QWidget * gen_d_drawer_get_content(DDrawer *self) { return self->getContent(); }
void gen_d_drawer_set_header_height(DDrawer *self, int32_t height) { self->setHeaderHeight(height); }
void gen_d_drawer_set_expand(DDrawer *self, bool value) { self->setExpand(value); }
bool gen_d_drawer_expand(DDrawer *self) { return self->expand(); }
void gen_d_drawer_set_animation_duration(DDrawer *self, int32_t duration) { self->setAnimationDuration(duration); }
void gen_d_drawer_set_separator_visible(DDrawer *self, bool arg) { self->setSeparatorVisible(arg); }
void gen_d_drawer_set_expanded_separator_visible(DDrawer *self, bool arg) { self->setExpandedSeparatorVisible(arg); }
DDrawerGroup *gen_d_drawer_group_new() { return new DDrawerGroup; }
DDrawer * gen_d_drawer_group_checked_expand(DDrawerGroup *self) { return self->checkedExpand(); }
DDrawer * gen_d_drawer_group_expand(DDrawerGroup *self, int32_t id) { return self->expand(id); }
int32_t gen_d_drawer_group_checked_id(DDrawerGroup *self) { return self->checkedId(); }
DFileChooserEdit *gen_d_file_chooser_edit_new() { return new DFileChooserEdit; }
void gen_d_file_chooser_edit_set_dialog_display_position(DFileChooserEdit *self, int32_t dialogDisplayPosition) { self->setDialogDisplayPosition(static_cast<DFileChooserEdit::DialogDisplayPosition>(dialogDisplayPosition)); }
int32_t gen_d_file_chooser_edit_dialog_display_position(DFileChooserEdit *self) { return static_cast<int32_t>(self->dialogDisplayPosition()); }
void gen_d_file_chooser_edit_init_dialog(DFileChooserEdit *self) { self->initDialog(); }
void gen_d_file_dialog_add_line_edit(DFileDialog *self, rust::Str text) { self->addLineEdit(from_rust_str(text)); }
void gen_d_file_dialog_set_allow_mixed_selection(DFileDialog *self, bool on) { self->setAllowMixedSelection(on); }
rust::String gen_d_file_dialog_get_combo_box_value(DFileDialog *self, rust::Str text) { return to_rust_string(self->getComboBoxValue(from_rust_str(text))); }
rust::String gen_d_file_dialog_get_line_edit_value(DFileDialog *self, rust::Str text) { return to_rust_string(self->getLineEditValue(from_rust_str(text))); }
void gen_d_file_dialog_set_visible(DFileDialog *self, bool visible) { self->setVisible(visible); }
DFontComboBox *gen_d_font_combo_box_new() { return new DFontComboBox; }
QFont * gen_d_font_combo_box_current_font(DFontComboBox *self) { return new QFont(self->currentFont()); }
QSize * gen_d_font_combo_box_size_hint(DFontComboBox *self) { return new QSize(self->sizeHint()); }
void gen_d_font_combo_box_set_current_font(DFontComboBox *self, QFont * f) { self->setCurrentFont(*f); }
DGraphicsGlowEffect *gen_d_graphics_glow_effect_new() { return new DGraphicsGlowEffect; }
DHeaderLine *gen_d_header_line_new() { return new DHeaderLine; }
void gen_d_header_line_set_title(DHeaderLine *self, rust::Str title) { self->setTitle(from_rust_str(title)); }
rust::String gen_d_header_line_title(DHeaderLine *self) { return to_rust_string(self->title()); }
DImageViewer *gen_d_image_viewer_new() { return new DImageViewer; }
rust::String gen_d_image_viewer_file_name(DImageViewer *self) { return to_rust_string(self->fileName()); }
void gen_d_image_viewer_set_file_name(DImageViewer *self, rust::Str fileName) { self->setFileName(from_rust_str(fileName)); }
double gen_d_image_viewer_scale_factor(DImageViewer *self) { return self->scaleFactor(); }
void gen_d_image_viewer_set_scale_factor(DImageViewer *self, double factor) { self->setScaleFactor(factor); }
void gen_d_image_viewer_scale_image(DImageViewer *self, double factor) { self->scaleImage(factor); }
void gen_d_image_viewer_auto_fit_image(DImageViewer *self) { self->autoFitImage(); }
void gen_d_image_viewer_fit_to_widget(DImageViewer *self) { self->fitToWidget(); }
void gen_d_image_viewer_fit_normal_size(DImageViewer *self) { self->fitNormalSize(); }
void gen_d_image_viewer_rotate_clockwise(DImageViewer *self) { self->rotateClockwise(); }
void gen_d_image_viewer_rotate_counterclockwise(DImageViewer *self) { self->rotateCounterclockwise(); }
int32_t gen_d_image_viewer_rotate_angle(DImageViewer *self) { return self->rotateAngle(); }
void gen_d_image_viewer_reset_rotate_angle(DImageViewer *self) { self->resetRotateAngle(); }
void gen_d_image_viewer_clear(DImageViewer *self) { self->clear(); }
void gen_d_image_viewer_center_on(DImageViewer *self, double x, double y) { self->centerOn(x, y); }
QRect * gen_d_image_viewer_visible_image_rect(DImageViewer *self) { return new QRect(self->visibleImageRect()); }
void gen_d_image_viewer_begin_crop_image(DImageViewer *self) { self->beginCropImage(); }
void gen_d_image_viewer_end_crop_image(DImageViewer *self) { self->endCropImage(); }
void gen_d_image_viewer_reset_crop_image(DImageViewer *self) { self->resetCropImage(); }
void gen_d_image_viewer_set_crop_aspect_ratio(DImageViewer *self, double w, double h) { self->setCropAspectRatio(w, h); }
QRect * gen_d_image_viewer_crop_image_rect(DImageViewer *self) { return new QRect(self->cropImageRect()); }
DIpv4LineEdit *gen_d_ipv4_line_edit_new() { return new DIpv4LineEdit; }
rust::String gen_d_ipv4_line_edit_display_text(DIpv4LineEdit *self) { return to_rust_string(self->displayText()); }
int32_t gen_d_ipv4_line_edit_cursor_position(DIpv4LineEdit *self) { return self->cursorPosition(); }
int32_t gen_d_ipv4_line_edit_alignment(DIpv4LineEdit *self) { return (self->alignment()).toInt(); }
bool gen_d_ipv4_line_edit_has_acceptable_input(DIpv4LineEdit *self) { return self->hasAcceptableInput(); }
bool gen_d_ipv4_line_edit_is_read_only(DIpv4LineEdit *self) { return self->isReadOnly(); }
void gen_d_ipv4_line_edit_set_cursor_position(DIpv4LineEdit *self, int32_t cursorPosition) { self->setCursorPosition(cursorPosition); }
void gen_d_ipv4_line_edit_set_read_only(DIpv4LineEdit *self, bool readOnly) { self->setReadOnly(readOnly); }
void gen_d_ipv4_line_edit_set_selection(DIpv4LineEdit *self, int32_t start, int32_t length) { self->setSelection(start, length); }
void gen_d_ipv4_line_edit_select_all(DIpv4LineEdit *self) { self->selectAll(); }
DKeySequenceEdit *gen_d_key_sequence_edit_new() { return new DKeySequenceEdit; }
void gen_d_key_sequence_edit_clear(DKeySequenceEdit *self) { self->clear(); }
void gen_d_key_sequence_edit_shortcut_direction(DKeySequenceEdit *self, int32_t alig) { self->ShortcutDirection(static_cast<Qt::AlignmentFlag>(alig)); }
DLineEdit *gen_d_line_edit_new() { return new DLineEdit; }
void gen_d_line_edit_set_placeholder_text(DLineEdit *self, rust::Str arg0) { self->setPlaceholderText(from_rust_str(arg0)); }
void gen_d_line_edit_set_alert(DLineEdit *self, bool isAlert) { self->setAlert(isAlert); }
bool gen_d_line_edit_is_alert(DLineEdit *self) { return self->isAlert(); }
void gen_d_line_edit_show_alert_message(DLineEdit *self, rust::Str text, int32_t duration) { self->showAlertMessage(from_rust_str(text), duration); }
void gen_d_line_edit_set_alert_message_alignment(DLineEdit *self, int32_t alignment) { self->setAlertMessageAlignment(Qt::Alignment::fromInt(alignment)); }
int32_t gen_d_line_edit_alert_message_alignment(DLineEdit *self) { return (self->alertMessageAlignment()).toInt(); }
void gen_d_line_edit_hide_alert_message(DLineEdit *self) { self->hideAlertMessage(); }
void gen_d_line_edit_set_left_widgets_visible(DLineEdit *self, bool visible) { self->setLeftWidgetsVisible(visible); }
void gen_d_line_edit_set_right_widgets_visible(DLineEdit *self, bool visible) { self->setRightWidgetsVisible(visible); }
void gen_d_line_edit_set_clear_button_enabled(DLineEdit *self, bool enable) { self->setClearButtonEnabled(enable); }
bool gen_d_line_edit_is_clear_button_enabled(DLineEdit *self) { return self->isClearButtonEnabled(); }
void gen_d_line_edit_set_text(DLineEdit *self, rust::Str text) { self->setText(from_rust_str(text)); }
rust::String gen_d_line_edit_text(DLineEdit *self) { return to_rust_string(self->text()); }
void gen_d_line_edit_clear(DLineEdit *self) { self->clear(); }
void gen_d_line_edit_set_context_menu_policy(DLineEdit *self, int32_t policy) { self->setContextMenuPolicy(static_cast<Qt::ContextMenuPolicy>(policy)); }
bool gen_d_line_edit_speech_to_text_is_enabled(DLineEdit *self) { return self->speechToTextIsEnabled(); }
void gen_d_line_edit_set_speech_to_text_enabled(DLineEdit *self, bool enable) { self->setSpeechToTextEnabled(enable); }
bool gen_d_line_edit_text_to_speech_is_enabled(DLineEdit *self) { return self->textToSpeechIsEnabled(); }
void gen_d_line_edit_set_text_to_speech_enabled(DLineEdit *self, bool enable) { self->setTextToSpeechEnabled(enable); }
bool gen_d_line_edit_text_to_translate_is_enabled(DLineEdit *self) { return self->textToTranslateIsEnabled(); }
void gen_d_line_edit_set_text_to_translate_enabled(DLineEdit *self, bool enable) { self->setTextToTranslateEnabled(enable); }
bool gen_d_line_edit_copy_enabled(DLineEdit *self) { return self->copyEnabled(); }
void gen_d_line_edit_set_copy_enabled(DLineEdit *self, bool enable) { self->setCopyEnabled(enable); }
bool gen_d_line_edit_cut_enabled(DLineEdit *self) { return self->cutEnabled(); }
void gen_d_line_edit_set_cut_enabled(DLineEdit *self, bool enable) { self->setCutEnabled(enable); }
bool gen_d_line_edit_paste_enabled(DLineEdit *self) { return self->pasteEnabled(); }
void gen_d_line_edit_set_paste_enabled(DLineEdit *self, bool enable) { self->setPasteEnabled(enable); }
DListView *gen_d_list_view_new() { return new DListView; }
QWidget * gen_d_list_view_get_header_widget(DListView *self, int32_t index) { return self->getHeaderWidget(index); }
QWidget * gen_d_list_view_get_footer_widget(DListView *self, int32_t index) { return self->getFooterWidget(index); }
bool gen_d_list_view_is_active_rect(DListView *self, QRect * rect) { return self->isActiveRect(*rect); }
bool gen_d_list_view_is_visual_rect(DListView *self, QRect * rect) { return self->isVisualRect(*rect); }
int32_t gen_d_list_view_count(DListView *self) { return self->count(); }
int32_t gen_d_list_view_orientation(DListView *self) { return static_cast<int32_t>(self->orientation()); }
QSize * gen_d_list_view_minimum_size_hint(DListView *self) { return new QSize(self->minimumSizeHint()); }
QSize * gen_d_list_view_item_size(DListView *self) { return new QSize(self->itemSize()); }
bool gen_d_list_view_remove_item(DListView *self, int32_t index) { return self->removeItem(index); }
bool gen_d_list_view_remove_items(DListView *self, int32_t index, int32_t count) { return self->removeItems(index, count); }
void gen_d_list_view_remove_header_widget(DListView *self, int32_t index) { self->removeHeaderWidget(index); }
QWidget * gen_d_list_view_take_header_widget(DListView *self, int32_t index) { return self->takeHeaderWidget(index); }
void gen_d_list_view_remove_footer_widget(DListView *self, int32_t index) { self->removeFooterWidget(index); }
QWidget * gen_d_list_view_take_footer_widget(DListView *self, int32_t index) { return self->takeFooterWidget(index); }
void gen_d_list_view_set_item_size(DListView *self, QSize * itemSize) { self->setItemSize(*itemSize); }
void gen_d_list_view_set_item_spacing(DListView *self, int32_t spacing) { self->setItemSpacing(spacing); }
void gen_d_list_view_set_item_radius(DListView *self, int32_t radius) { self->setItemRadius(radius); }
DLoadingIndicator *gen_d_loading_indicator_new() { return new DLoadingIndicator; }
QColor * gen_d_loading_indicator_background_color(DLoadingIndicator *self) { return new QColor(self->backgroundColor()); }
bool gen_d_loading_indicator_loading(DLoadingIndicator *self) { return self->loading(); }
QWidget * gen_d_loading_indicator_widget_source(DLoadingIndicator *self) { return self->widgetSource(); }
QPixmap * gen_d_loading_indicator_image_source(DLoadingIndicator *self) { return new QPixmap(self->imageSource()); }
int32_t gen_d_loading_indicator_ani_duration(DLoadingIndicator *self) { return self->aniDuration(); }
QSize * gen_d_loading_indicator_size_hint(DLoadingIndicator *self) { return new QSize(self->sizeHint()); }
bool gen_d_loading_indicator_smooth(DLoadingIndicator *self) { return self->smooth(); }
int32_t gen_d_loading_indicator_direction(DLoadingIndicator *self) { return static_cast<int32_t>(self->direction()); }
double gen_d_loading_indicator_rotate(DLoadingIndicator *self) { return self->rotate(); }
void gen_d_loading_indicator_start(DLoadingIndicator *self) { self->start(); }
void gen_d_loading_indicator_stop(DLoadingIndicator *self) { self->stop(); }
void gen_d_loading_indicator_set_loading(DLoadingIndicator *self, bool flag) { self->setLoading(flag); }
void gen_d_loading_indicator_set_ani_duration(DLoadingIndicator *self, int32_t msecs) { self->setAniDuration(msecs); }
void gen_d_loading_indicator_set_background_color(DLoadingIndicator *self, QColor * color) { self->setBackgroundColor(*color); }
void gen_d_loading_indicator_set_widget_source(DLoadingIndicator *self, QWidget * widgetSource) { self->setWidgetSource(widgetSource); }
void gen_d_loading_indicator_set_image_source(DLoadingIndicator *self, QPixmap * imageSource) { self->setImageSource(*imageSource); }
void gen_d_loading_indicator_set_smooth(DLoadingIndicator *self, bool smooth) { self->setSmooth(smooth); }
void gen_d_loading_indicator_set_direction(DLoadingIndicator *self, int32_t direction) { self->setDirection(static_cast<DLoadingIndicator::RotationDirection>(direction)); }
DMPRISControl *gen_d_m_p_r_i_s_control_new() { return new DMPRISControl; }
bool gen_d_m_p_r_i_s_control_is_working(DMPRISControl *self) { return self->isWorking(); }
void gen_d_m_p_r_i_s_control_set_picture_visible(DMPRISControl *self, bool visible) { self->setPictureVisible(visible); }
void gen_d_m_p_r_i_s_control_set_picture_size(DMPRISControl *self, QSize * size) { self->setPictureSize(*size); }
DPageIndicator *gen_d_page_indicator_new() { return new DPageIndicator; }
int32_t gen_d_page_indicator_page_count(DPageIndicator *self) { return self->pageCount(); }
void gen_d_page_indicator_set_page_count(DPageIndicator *self, int32_t count) { self->setPageCount(count); }
void gen_d_page_indicator_next_page(DPageIndicator *self) { self->nextPage(); }
void gen_d_page_indicator_previous_page(DPageIndicator *self) { self->previousPage(); }
void gen_d_page_indicator_set_current_page(DPageIndicator *self, int32_t index) { self->setCurrentPage(index); }
int32_t gen_d_page_indicator_current_page_index(DPageIndicator *self) { return self->currentPageIndex(); }
QColor * gen_d_page_indicator_point_color(DPageIndicator *self) { return new QColor(self->pointColor()); }
void gen_d_page_indicator_set_point_color(DPageIndicator *self, QColor * color) { self->setPointColor(*color); }
QColor * gen_d_page_indicator_secondary_point_color(DPageIndicator *self) { return new QColor(self->secondaryPointColor()); }
void gen_d_page_indicator_set_secondary_point_color(DPageIndicator *self, QColor * color) { self->setSecondaryPointColor(*color); }
int32_t gen_d_page_indicator_point_radius(DPageIndicator *self) { return self->pointRadius(); }
void gen_d_page_indicator_set_point_radius(DPageIndicator *self, int32_t size) { self->setPointRadius(size); }
int32_t gen_d_page_indicator_secondary_point_radius(DPageIndicator *self) { return self->secondaryPointRadius(); }
void gen_d_page_indicator_set_secondary_point_radius(DPageIndicator *self, int32_t size) { self->setSecondaryPointRadius(size); }
int32_t gen_d_page_indicator_point_distance(DPageIndicator *self) { return self->pointDistance(); }
void gen_d_page_indicator_set_point_distance(DPageIndicator *self, int32_t distance) { self->setPointDistance(distance); }
DPasswordEdit *gen_d_password_edit_new() { return new DPasswordEdit; }
bool gen_d_password_edit_is_echo_mode(DPasswordEdit *self) { return self->isEchoMode(); }
void gen_d_password_edit_set_echo_button_is_visible(DPasswordEdit *self, bool visible) { self->setEchoButtonIsVisible(visible); }
bool gen_d_password_edit_echo_button_is_visible(DPasswordEdit *self) { return self->echoButtonIsVisible(); }
DPictureSequenceView *gen_d_picture_sequence_view_new() { return new DPictureSequenceView; }
void gen_d_picture_sequence_view_play(DPictureSequenceView *self) { self->play(); }
void gen_d_picture_sequence_view_pause(DPictureSequenceView *self) { self->pause(); }
void gen_d_picture_sequence_view_stop(DPictureSequenceView *self) { self->stop(); }
int32_t gen_d_picture_sequence_view_speed(DPictureSequenceView *self) { return self->speed(); }
void gen_d_picture_sequence_view_set_speed(DPictureSequenceView *self, int32_t speed) { self->setSpeed(speed); }
bool gen_d_picture_sequence_view_single_shot(DPictureSequenceView *self) { return self->singleShot(); }
void gen_d_picture_sequence_view_set_single_shot(DPictureSequenceView *self, bool singleShot) { self->setSingleShot(singleShot); }
void gen_d_print_preview_widget_set_visible(DPrintPreviewWidget *self, bool visible) { self->setVisible(visible); }
void gen_d_print_preview_widget_set_page_range(DPrintPreviewWidget *self, int32_t from, int32_t to) { self->setPageRange(from, to); }
void gen_d_print_preview_widget_set_page_range_a_l_l(DPrintPreviewWidget *self) { self->setPageRangeALL(); }
void gen_d_print_preview_widget_set_page_range_mode(DPrintPreviewWidget *self, int32_t mode) { self->setPageRangeMode(static_cast<DPrintPreviewWidget::PageRange>(mode)); }
int32_t gen_d_print_preview_widget_page_range_mode(DPrintPreviewWidget *self) { return static_cast<int32_t>(self->pageRangeMode()); }
int32_t gen_d_print_preview_widget_pages_count(DPrintPreviewWidget *self) { return self->pagesCount(); }
int32_t gen_d_print_preview_widget_current_page(DPrintPreviewWidget *self) { return self->currentPage(); }
bool gen_d_print_preview_widget_turn_page_able(DPrintPreviewWidget *self) { return self->turnPageAble(); }
void gen_d_print_preview_widget_set_scale(DPrintPreviewWidget *self, double scale) { self->setScale(scale); }
double gen_d_print_preview_widget_get_scale(DPrintPreviewWidget *self) { return self->getScale(); }
void gen_d_print_preview_widget_update_view(DPrintPreviewWidget *self) { self->updateView(); }
void gen_d_print_preview_widget_update_water_mark(DPrintPreviewWidget *self) { self->updateWaterMark(); }
void gen_d_print_preview_widget_refresh_begin(DPrintPreviewWidget *self) { self->refreshBegin(); }
void gen_d_print_preview_widget_refresh_end(DPrintPreviewWidget *self) { self->refreshEnd(); }
void gen_d_print_preview_widget_set_water_mark_type(DPrintPreviewWidget *self, int32_t type_) { self->setWaterMarkType(type_); }
void gen_d_print_preview_widget_set_water_mark_rotate(DPrintPreviewWidget *self, double rotate) { self->setWaterMarkRotate(rotate); }
void gen_d_print_preview_widget_set_water_mark_scale(DPrintPreviewWidget *self, double scale) { self->setWaterMarkScale(scale); }
void gen_d_print_preview_widget_set_water_mark_opacity(DPrintPreviewWidget *self, double opacity) { self->setWaterMarkOpacity(opacity); }
void gen_d_print_preview_widget_set_confidential_water_mark(DPrintPreviewWidget *self) { self->setConfidentialWaterMark(); }
void gen_d_print_preview_widget_set_draft_water_mark(DPrintPreviewWidget *self) { self->setDraftWaterMark(); }
void gen_d_print_preview_widget_set_sample_water_mark(DPrintPreviewWidget *self) { self->setSampleWaterMark(); }
void gen_d_print_preview_widget_set_custom_water_mark(DPrintPreviewWidget *self, rust::Str text) { self->setCustomWaterMark(from_rust_str(text)); }
void gen_d_print_preview_widget_set_text_water_mark(DPrintPreviewWidget *self, rust::Str text) { self->setTextWaterMark(from_rust_str(text)); }
void gen_d_print_preview_widget_set_water_mark_font(DPrintPreviewWidget *self, QFont * font) { self->setWaterMarkFont(*font); }
QColor * gen_d_print_preview_widget_water_mark_color(DPrintPreviewWidget *self) { return new QColor(self->waterMarkColor()); }
void gen_d_print_preview_widget_set_water_mark_color(DPrintPreviewWidget *self, QColor * color) { self->setWaterMarkColor(*color); }
void gen_d_print_preview_widget_set_water_mark_layout(DPrintPreviewWidget *self, int32_t layout) { self->setWaterMarkLayout(layout); }
void gen_d_print_preview_widget_set_imposition(DPrintPreviewWidget *self, int32_t im) { self->setImposition(static_cast<DPrintPreviewWidget::Imposition>(im)); }
int32_t gen_d_print_preview_widget_imposition(DPrintPreviewWidget *self) { return static_cast<int32_t>(self->imposition()); }
void gen_d_print_preview_widget_set_order(DPrintPreviewWidget *self, int32_t order) { self->setOrder(static_cast<DPrintPreviewWidget::Order>(order)); }
int32_t gen_d_print_preview_widget_order(DPrintPreviewWidget *self) { return static_cast<int32_t>(self->order()); }
void gen_d_print_preview_widget_set_print_from_path(DPrintPreviewWidget *self, rust::Str path) { self->setPrintFromPath(from_rust_str(path)); }
rust::String gen_d_print_preview_widget_print_from_path(DPrintPreviewWidget *self) { return to_rust_string(self->printFromPath()); }
void gen_d_print_preview_widget_set_print_mode(DPrintPreviewWidget *self, int32_t pt) { self->setPrintMode(static_cast<DPrintPreviewWidget::PrintMode>(pt)); }
void gen_d_print_preview_widget_set_asyn_preview(DPrintPreviewWidget *self, int32_t totalPage) { self->setAsynPreview(totalPage); }
bool gen_d_print_preview_widget_is_asyn_preview(DPrintPreviewWidget *self) { return self->isAsynPreview(); }
void gen_d_print_preview_widget_is_page_by_page(DPrintPreviewWidget *self, int32_t pageCopy, bool isFirst) { self->isPageByPage(pageCopy, isFirst); }
int32_t gen_d_print_preview_widget_target_page_count(DPrintPreviewWidget *self, int32_t pageCount) { return self->targetPageCount(pageCount); }
int32_t gen_d_print_preview_widget_origin_page_count(DPrintPreviewWidget *self) { return self->originPageCount(); }
rust::String gen_d_print_preview_widget_printer_color_model(DPrintPreviewWidget *self) { return to_rust_string(self->printerColorModel()); }
void gen_d_print_preview_widget_update_preview(DPrintPreviewWidget *self) { self->updatePreview(); }
void gen_d_print_preview_widget_turn_front(DPrintPreviewWidget *self) { self->turnFront(); }
void gen_d_print_preview_widget_turn_back(DPrintPreviewWidget *self) { self->turnBack(); }
void gen_d_print_preview_widget_turn_begin(DPrintPreviewWidget *self) { self->turnBegin(); }
void gen_d_print_preview_widget_turn_end(DPrintPreviewWidget *self) { self->turnEnd(); }
void gen_d_print_preview_widget_set_current_page(DPrintPreviewWidget *self, int32_t page) { self->setCurrentPage(page); }
void gen_d_print_preview_widget_print(DPrintPreviewWidget *self, bool isSavedPicture) { self->print(isSavedPicture); }
DSearchComboBox *gen_d_search_combo_box_new() { return new DSearchComboBox; }
void gen_d_search_combo_box_set_editable(DSearchComboBox *self, bool editable) { self->setEditable(editable); }
DSearchEdit *gen_d_search_edit_new() { return new DSearchEdit; }
void gen_d_search_edit_set_place_holder(DSearchEdit *self, rust::Str placeHolder) { self->setPlaceHolder(from_rust_str(placeHolder)); }
rust::String gen_d_search_edit_place_holder(DSearchEdit *self) { return to_rust_string(self->placeHolder()); }
void gen_d_search_edit_clear(DSearchEdit *self) { self->clear(); }
void gen_d_search_edit_clear_edit(DSearchEdit *self) { self->clearEdit(); }
bool gen_d_search_edit_is_voice_input(DSearchEdit *self) { return self->isVoiceInput(); }
void gen_d_search_edit_set_placeholder_text(DSearchEdit *self, rust::Str text) { self->setPlaceholderText(from_rust_str(text)); }
rust::String gen_d_search_edit_placeholder_text(DSearchEdit *self) { return to_rust_string(self->placeholderText()); }
DSettingsDialog *gen_d_settings_dialog_new() { return new DSettingsDialog; }
DSettingsWidgetFactory * gen_d_settings_dialog_widget_factory(DSettingsDialog *self) { return self->widgetFactory(); }
bool gen_d_settings_dialog_group_is_visible(DSettingsDialog *self, rust::Str groupKey) { return self->groupIsVisible(from_rust_str(groupKey)); }
void gen_d_settings_dialog_set_reset_visible(DSettingsDialog *self, bool visible) { self->setResetVisible(visible); }
void gen_d_settings_dialog_scroll_to_group(DSettingsDialog *self, rust::Str groupKey) { self->scrollToGroup(from_rust_str(groupKey)); }
void gen_d_settings_dialog_set_icon(DSettingsDialog *self, QIcon * icon) { self->setIcon(*icon); }
void gen_d_settings_dialog_set_group_visible(DSettingsDialog *self, rust::Str groupKey, bool visible) { self->setGroupVisible(from_rust_str(groupKey), visible); }
DSettingsWidgetFactory *gen_d_settings_widget_factory_new() { return new DSettingsWidgetFactory; }
DShadowLine *gen_d_shadow_line_new() { return new DShadowLine; }
QSize * gen_d_shadow_line_size_hint(DShadowLine *self) { return new QSize(self->sizeHint()); }
DSimpleListView *gen_d_simple_list_view_new() { return new DSimpleListView; }
void gen_d_simple_list_view_set_row_height(DSimpleListView *self, int32_t height) { self->setRowHeight(height); }
void gen_d_simple_list_view_set_clip_radius(DSimpleListView *self, int32_t radius) { self->setClipRadius(radius); }
void gen_d_simple_list_view_remove_item(DSimpleListView *self, DSimpleListItem * item) { self->removeItem(item); }
void gen_d_simple_list_view_clear_items(DSimpleListView *self) { self->clearItems(); }
void gen_d_simple_list_view_clear_selections(DSimpleListView *self, bool clearLastSelection) { self->clearSelections(clearLastSelection); }
void gen_d_simple_list_view_search(DSimpleListView *self, rust::Str searchContent) { self->search(from_rust_str(searchContent)); }
void gen_d_simple_list_view_set_single_select(DSimpleListView *self, bool singleSelect) { self->setSingleSelect(singleSelect); }
void gen_d_simple_list_view_keep_select_when_click_blank(DSimpleListView *self, bool keep) { self->keepSelectWhenClickBlank(keep); }
void gen_d_simple_list_view_select_all_items(DSimpleListView *self) { self->selectAllItems(); }
void gen_d_simple_list_view_select_first_item(DSimpleListView *self) { self->selectFirstItem(); }
void gen_d_simple_list_view_select_last_item(DSimpleListView *self) { self->selectLastItem(); }
void gen_d_simple_list_view_select_next_item(DSimpleListView *self) { self->selectNextItem(); }
void gen_d_simple_list_view_select_prev_item(DSimpleListView *self) { self->selectPrevItem(); }
void gen_d_simple_list_view_shift_select_page_down(DSimpleListView *self) { self->shiftSelectPageDown(); }
void gen_d_simple_list_view_shift_select_page_up(DSimpleListView *self) { self->shiftSelectPageUp(); }
void gen_d_simple_list_view_shift_select_to_end(DSimpleListView *self) { self->shiftSelectToEnd(); }
void gen_d_simple_list_view_shift_select_to_home(DSimpleListView *self) { self->shiftSelectToHome(); }
void gen_d_simple_list_view_shift_select_to_next(DSimpleListView *self) { self->shiftSelectToNext(); }
void gen_d_simple_list_view_shift_select_to_prev(DSimpleListView *self) { self->shiftSelectToPrev(); }
void gen_d_simple_list_view_scroll_page_down(DSimpleListView *self) { self->scrollPageDown(); }
void gen_d_simple_list_view_scroll_page_up(DSimpleListView *self) { self->scrollPageUp(); }
void gen_d_simple_list_view_ctrl_scroll_page_down(DSimpleListView *self) { self->ctrlScrollPageDown(); }
void gen_d_simple_list_view_ctrl_scroll_page_up(DSimpleListView *self) { self->ctrlScrollPageUp(); }
void gen_d_simple_list_view_ctrl_scroll_to_end(DSimpleListView *self) { self->ctrlScrollToEnd(); }
void gen_d_simple_list_view_ctrl_scroll_to_home(DSimpleListView *self) { self->ctrlScrollToHome(); }
DSlider *gen_d_slider_new() { return new DSlider; }
int32_t gen_d_slider_orientation(DSlider *self) { return static_cast<int32_t>(self->orientation()); }
void gen_d_slider_set_left_icon(DSlider *self, QIcon * left) { self->setLeftIcon(*left); }
void gen_d_slider_set_right_icon(DSlider *self, QIcon * right) { self->setRightIcon(*right); }
void gen_d_slider_set_icon_size(DSlider *self, QSize * size) { self->setIconSize(*size); }
void gen_d_slider_set_minimum(DSlider *self, int32_t min) { self->setMinimum(min); }
int32_t gen_d_slider_minimum(DSlider *self) { return self->minimum(); }
void gen_d_slider_set_value(DSlider *self, int32_t value) { self->setValue(value); }
int32_t gen_d_slider_value(DSlider *self) { return self->value(); }
void gen_d_slider_set_page_step(DSlider *self, int32_t pageStep) { self->setPageStep(pageStep); }
int32_t gen_d_slider_page_step(DSlider *self) { return self->pageStep(); }
void gen_d_slider_set_maximum(DSlider *self, int32_t max) { self->setMaximum(max); }
int32_t gen_d_slider_maximum(DSlider *self) { return self->maximum(); }
void gen_d_slider_set_mouse_wheel_enabled(DSlider *self, bool enabled) { self->setMouseWheelEnabled(enabled); }
void gen_d_slider_set_tip_value(DSlider *self, rust::Str value) { self->setTipValue(from_rust_str(value)); }
QSize * gen_d_slider_size_hint(DSlider *self) { return new QSize(self->sizeHint()); }
void gen_d_slider_set_handle_visible(DSlider *self, bool b) { self->setHandleVisible(b); }
bool gen_d_slider_handle_visible(DSlider *self) { return self->handleVisible(); }
void gen_d_slider_set_enabled_across_style(DSlider *self, bool enabled) { self->setEnabledAcrossStyle(enabled); }
DSpinBox *gen_d_spin_box_new() { return new DSpinBox; }
bool gen_d_spin_box_is_alert(DSpinBox *self) { return self->isAlert(); }
void gen_d_spin_box_show_alert_message(DSpinBox *self, rust::Str text, int32_t duration) { self->showAlertMessage(from_rust_str(text), duration); }
void gen_d_spin_box_set_enabled_embed_style(DSpinBox *self, bool enabled) { self->setEnabledEmbedStyle(enabled); }
void gen_d_spin_box_set_alert(DSpinBox *self, bool alert) { self->setAlert(alert); }
DDoubleSpinBox *gen_d_double_spin_box_new() { return new DDoubleSpinBox; }
bool gen_d_double_spin_box_is_alert(DDoubleSpinBox *self) { return self->isAlert(); }
void gen_d_double_spin_box_show_alert_message(DDoubleSpinBox *self, rust::Str text, int32_t duration) { self->showAlertMessage(from_rust_str(text), duration); }
void gen_d_double_spin_box_set_enabled_embed_style(DDoubleSpinBox *self, bool enabled) { self->setEnabledEmbedStyle(enabled); }
void gen_d_double_spin_box_set_alert(DDoubleSpinBox *self, bool alert) { self->setAlert(alert); }
DSpinner *gen_d_spinner_new() { return new DSpinner; }
bool gen_d_spinner_is_playing(DSpinner *self) { return self->isPlaying(); }
void gen_d_spinner_start(DSpinner *self) { self->start(); }
void gen_d_spinner_stop(DSpinner *self) { self->stop(); }
void gen_d_spinner_set_background_color(DSpinner *self, QColor * color) { self->setBackgroundColor(*color); }
DSwitchButton *gen_d_switch_button_new() { return new DSwitchButton; }
QSize * gen_d_switch_button_size_hint(DSwitchButton *self) { return new QSize(self->sizeHint()); }
DSwitchLineExpand *gen_d_switch_line_expand_new() { return new DSwitchLineExpand; }
void gen_d_switch_line_expand_set_title(DSwitchLineExpand *self, rust::Str title) { self->setTitle(from_rust_str(title)); }
void gen_d_switch_line_expand_set_expand(DSwitchLineExpand *self, bool value) { self->setExpand(value); }
DBaseLine * gen_d_switch_line_expand_header(DSwitchLineExpand *self) { return self->header(); }
DTabletWindowOptionButton *gen_d_tablet_window_option_button_new() { return new DTabletWindowOptionButton; }
QSize * gen_d_tablet_window_option_button_size_hint(DTabletWindowOptionButton *self) { return new QSize(self->sizeHint()); }
void gen_d_tick_effect_play(DTickEffect *self) { self->play(); }
void gen_d_tick_effect_stop(DTickEffect *self) { self->stop(); }
void gen_d_tick_effect_pause(DTickEffect *self) { self->pause(); }
void gen_d_tick_effect_resume(DTickEffect *self) { self->resume(); }
void gen_d_tick_effect_set_direction(DTickEffect *self, int32_t direction) { self->setDirection(static_cast<DTickEffect::Direction>(direction)); }
void gen_d_tick_effect_set_fixed_pixel_move(DTickEffect *self, int32_t pixel) { self->setFixedPixelMove(pixel); }
DTipLabel *gen_d_tip_label_new() { return new DTipLabel; }
void gen_d_tip_label_show(DTipLabel *self, QPoint * pos) { self->show(*pos); }
rust::String gen_d_titlebar_tool_base_interface_id(DTitlebarToolBaseInterface *self) { return to_rust_string(self->id()); }
rust::String gen_d_titlebar_tool_base_interface_description(DTitlebarToolBaseInterface *self) { return to_rust_string(self->description()); }
rust::String gen_d_titlebar_tool_base_interface_icon_name(DTitlebarToolBaseInterface *self) { return to_rust_string(self->iconName()); }
QWidget * gen_d_title_bar_tool_interface_create_view(DTitleBarToolInterface *self) { return self->createView(); }
QWidget * gen_d_title_bar_spacer_interface_create_placeholder_view(DTitleBarSpacerInterface *self) { return self->createPlaceholderView(); }
int32_t gen_d_title_bar_spacer_interface_size(DTitleBarSpacerInterface *self) { return self->size(); }
QWidget * gen_d_titlebar_settings_tools_edit_panel(DTitlebarSettings *self) { return self->toolsEditPanel(); }
DToolButton *gen_d_tool_button_new() { return new DToolButton; }
void gen_d_tool_button_set_alignment(DToolButton *self, int32_t flag) { self->setAlignment(Qt::Alignment::fromInt(flag)); }
int32_t gen_d_tool_button_alignment(DToolButton *self) { return (self->alignment()).toInt(); }
DWaterProgress *gen_d_water_progress_new() { return new DWaterProgress; }
int32_t gen_d_water_progress_value(DWaterProgress *self) { return self->value(); }
void gen_d_water_progress_start(DWaterProgress *self) { self->start(); }
void gen_d_water_progress_stop(DWaterProgress *self) { self->stop(); }
void gen_d_water_progress_set_value(DWaterProgress *self, int32_t value) { self->setValue(value); }
void gen_d_water_progress_set_text_visible(DWaterProgress *self, bool visible) { self->setTextVisible(visible); }
DWindowCloseButton *gen_d_window_close_button_new() { return new DWindowCloseButton; }
QSize * gen_d_window_close_button_size_hint(DWindowCloseButton *self) { return new QSize(self->sizeHint()); }
DWindowMaxButton *gen_d_window_max_button_new() { return new DWindowMaxButton; }
bool gen_d_window_max_button_is_maximized(DWindowMaxButton *self) { return self->isMaximized(); }
QSize * gen_d_window_max_button_size_hint(DWindowMaxButton *self) { return new QSize(self->sizeHint()); }
void gen_d_window_max_button_set_maximized(DWindowMaxButton *self, bool isMaximized) { self->setMaximized(isMaximized); }
DWindowMinButton *gen_d_window_min_button_new() { return new DWindowMinButton; }
QSize * gen_d_window_min_button_size_hint(DWindowMinButton *self) { return new QSize(self->sizeHint()); }
DWindowOptionButton *gen_d_window_option_button_new() { return new DWindowOptionButton; }
QSize * gen_d_window_option_button_size_hint(DWindowOptionButton *self) { return new QSize(self->sizeHint()); }
DWindowQuitFullButton *gen_d_window_quit_full_button_new() { return new DWindowQuitFullButton; }
QSize * gen_d_window_quit_full_button_size_hint(DWindowQuitFullButton *self) { return new QSize(self->sizeHint()); }

} // namespace dtkrs
