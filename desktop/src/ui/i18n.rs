#![allow(dead_code)]
use crate::config::AppLanguage;
use eframe::egui;
use std::sync::Arc;

pub(crate) fn configure_ui_fonts(context: &egui::Context, prefer_vazirmatn: bool) {
    let key = egui::Id::new("pakar-font-language");
    let previous = context.data(|data| data.get_temp::<bool>(key));
    if previous == Some(prefer_vazirmatn) {
        return;
    }
    // Initial install runs before the first frame. Later preference changes
    // keep fonts registered by widgets (notably Elegance's symbols). Starting
    // from defaults again would silently discard those families and glyphs.
    let mut fonts = if previous.is_some() {
        context.fonts(|fonts| fonts.definitions().clone())
    } else {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        fonts
    };

    // Prefer the platform UI face. Vazirmatn remains immediately behind it so
    // Persian and Arabic text has a bundled, release-safe fallback. For a
    // Persian UI the bundled font is first so shaping is consistent across OSes.
    let system_font = [
        #[cfg(target_os = "windows")]
        r"C:\Windows\Fonts\segoeui.ttf",
        #[cfg(target_os = "macos")]
        "/System/Library/Fonts/SFNS.ttf",
        #[cfg(target_os = "linux")]
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
        #[cfg(target_os = "linux")]
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ]
    .iter()
    .find_map(|path| std::fs::read(path).ok());

    fonts.font_data.insert(
        "pealayer-vazirmatn".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../../assets/fonts/Vazirmatn-Regular.ttf"
        ))),
    );
    if let Some(bytes) = system_font {
        fonts.font_data.insert(
            "pealayer-system-ui".to_owned(),
            Arc::new(egui::FontData::from_owned(bytes)),
        );
    }

    let has_system_font = fonts.font_data.contains_key("pealayer-system-ui");
    let proportional = fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .expect("egui provides a proportional font family");
    proportional.retain(|name| name != "pealayer-vazirmatn" && name != "pealayer-system-ui");
    if prefer_vazirmatn {
        proportional.insert(0, "pealayer-vazirmatn".to_owned());
        if has_system_font {
            proportional.insert(1, "pealayer-system-ui".to_owned());
        }
    } else if has_system_font {
        proportional.insert(0, "pealayer-system-ui".to_owned());
        proportional.insert(1, "pealayer-vazirmatn".to_owned());
    } else {
        proportional.insert(0, "pealayer-vazirmatn".to_owned());
    }

    // TextEdit and technical values use Monospace by default. The donor only
    // installed Proportional: retain Latin monospace but cover Persian there too.
    let monospace = fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap();
    if !monospace.iter().any(|name| name == "pealayer-vazirmatn") {
        monospace.push("pealayer-vazirmatn".to_owned());
    }
    context.set_fonts(fonts);
    context.data_mut(|data| data.insert_temp(key, prefer_vazirmatn));
}

/// Translate application-owned copy. Values learned from media metadata or
/// PCController are deliberately never passed through this table.
pub fn tr(language: AppLanguage, english: &str) -> String {
    if language != AppLanguage::Persian {
        return english.to_owned();
    }
    let translated = match english {
        "Update progress" => "پیشرفت به‌روزرسانی",
        "Disconnected" => "قطع‌شده",
        "Check hardware" => "بررسی سخت‌افزار",
        "Hardware" => "سخت‌افزار",
        "Network" => "شبکه",
        "No device selected" => "دستگاهی انتخاب نشده",
        "Manage connection" => "مدیریت اتصال",
        "No acknowledgement" => "بدون تأیید",
        "Progress not reported" => "پیشرفت گزارش نشده",
        "Active errors" => "خطاهای فعال",
        "Steady" => "ثابت",
        "Breathe" => "تنفسی",
        "Color fade" => "تغییر تدریجی رنگ",
        "Fade out" => "خاموش‌شدن تدریجی",
        "Pulse" => "تپش",
        "Double blink" => "چشمک دوتایی",
        "Revision" => "بازنگری",
        "Place at start" => "قرار دادن در ابتدا",
        "Place at end" => "قرار دادن در انتها",
        "Move earlier" => "انتقال به قبل",
        "Move later" => "انتقال به بعد",
        "Restore status bar" => "بازگردانی نوار وضعیت",
        "More status actions" => "عملیات بیشتر نوار وضعیت",
        "Copy color" => "کپی رنگ",
        "Status message" => "پیام وضعیت",
        "Lock red → white" => "قفل تغییر قرمز به سفید",
        "Board-enforced; Off remains available. The observed board value is shown only after ACK/readback." => {
            "قفل در خود برد اعمال می‌شود؛ خاموش‌کردن همچنان ممکن است. وضعیت فقط پس از تأیید و بازخوانی برد نمایش داده می‌شود."
        }
        "Import colors…" => "درون‌ریزی رنگ‌ها…",
        "Export colors" => "برون‌ریزی رنگ‌ها",
        "Import cell colors" => "درون‌ریزی رنگ خانه‌ها",
        "Export cell colors" => "برون‌ریزی رنگ خانه‌ها",
        "Cell assignments" => "رنگ‌های خانه‌ها",
        "Excel workbook (.xlsx)" => "کاربرگ اکسل (.xlsx)",
        "CSV (.csv)" => "پروندهٔ CSV (.csv)",
        "JSON (.json)" => "پروندهٔ JSON (.json)",
        "Review changes" => "بازبینی تغییرات",
        "Imported" => "درون‌ریزی‌شده",
        "assignments; omitted cells stay unchanged" => "خانه؛ خانه‌های ذکرنشده تغییر نمی‌کنند",
        "The selected board changed or is offline. Reopen this file after reconnecting." => {
            "برد انتخاب‌شده تغییر کرده یا آفلاین است. پس از اتصال دوباره پرونده را باز کنید."
        }
        "Red lock blocks this import. No assignments will be applied." => {
            "قفل قرمز مانع این درون‌ریزی است. هیچ خانه‌ای تغییر نخواهد کرد."
        }
        "Apply atomically" => "اعمال یکپارچه",
        "Waiting for a fresh board observation before export…" => {
            "در انتظار وضعیت تازهٔ برد برای برون‌ریزی…"
        }
        "Export cancelled: fresh board state was not available" => {
            "برون‌ریزی لغو شد: وضعیت تازهٔ برد در دسترس نبود"
        }
        "Exported all 80 observed assignments" => "رنگ مشاهده‌شدهٔ هر ۸۰ خانه برون‌ریزی شد",
        "Export failed" => "برون‌ریزی ناموفق بود",
        "Board sound" => "صدای برد",
        "Saved AVR settings" => "تنظیمات ذخیره‌شدهٔ AVR",
        "AVR name" => "نام AVR",
        "Play welcome melody" => "پخش ملودی خوشامدگویی",
        "Seven-segment brightness" => "روشنایی نمایشگر هفت‌قسمتی",
        "Button 9 ADC threshold" => "آستانهٔ ADC دکمهٔ ۹",
        "Periodic AVR telemetry" => "گزارش دوره‌ای AVR",
        "0 sends only changed state; otherwise 100–65535 ms." => {
            "صفر فقط تغییر وضعیت را ارسال می‌کند؛ در غیر این صورت ۱۰۰ تا ۶۵۵۳۵ میلی‌ثانیه."
        }
        "Start page" => "صفحهٔ آغازین",
        "Front-panel pages" => "صفحه‌های پنل جلویی",
        "Move page up" => "انتقال صفحه به بالا",
        "Move page down" => "انتقال صفحه به پایین",
        "Read board" => "خواندن برد",
        "Save to AVR" => "ذخیره در AVR",
        "Mutes the physical board buzzer, including welcome and menu sounds. Saved directly on the board; Windows notification sound is separate." => {
            "بیزر خود برد، از جمله صدای خوشامد و منوها را بی‌صدا می‌کند. مستقیم روی برد ذخیره می‌شود؛ صدای اعلان ویندوز جدا است."
        }
        "Mutes the physical board buzzer, including welcome and menu sounds. Saves directly on the board; browser sound is separate." => {
            "بیزر خود برد، از جمله صدای خوشامد و منوها را بی‌صدا می‌کند. مستقیم روی برد ذخیره می‌شود؛ صدای مرورگر جدا است."
        }
        "Appearance is saved in this browser. Board sound changes save directly to the board." => {
            "ظاهر در این مرورگر ذخیره می‌شود. تغییرات صدای برد مستقیم روی برد ذخیره می‌شوند."
        }
        "Programming safety keeps the board muted." => {
            "حفاظت هنگام برنامه‌ریزی، برد را بی‌صدا نگه می‌دارد."
        }
        "Verified · saved in EEPROM" => "تأیید شد · ذخیره‌شده در EEPROM",
        "Reading board…" => "در حال خواندن برد…",
        "LEDs & cells" => "LEDها و خانه‌ها",
        "AVR & inputs" => "AVR و ورودی‌ها",
        "Firmware & storage" => "میان‌افزار و ذخیره‌سازی",
        "Search settings…" => "جست‌وجوی تنظیمات…",
        "Close device settings" => "بستن تنظیمات دستگاه",
        "Device settings pages" => "صفحه‌های تنظیمات دستگاه",
        "Discard & reload…" => "لغو تغییرات و خواندن دوباره…",
        "Read the device to load settings" => "برای بارگذاری تنظیمات، دستگاه را بخوانید",
        "Only fresh, acknowledged device capabilities are shown. Application preferences are separate from board configuration." => {
            "فقط قابلیت‌های تازه و تأییدشدهٔ دستگاه نمایش داده می‌شوند. تنظیمات برنامه از پیکربندی برد جدا هستند."
        }
        "Firmware build information · read-only" => "اطلاعات ساخت میان‌افزار · فقط خواندنی",
        "UART flow-control mode, pin ownership and hardware layout are build flags, not runtime switches. Rebuild and safely update firmware to change them." => {
            "حالت کنترل جریان UART، تخصیص پایه‌ها و چیدمان سخت‌افزار گزینه‌های زمان ساخت هستند. برای تغییر آن‌ها، میان‌افزار را دوباره بسازید و به‌صورت ایمن به‌روز کنید."
        }
        "No observed build information" => "اطلاعات ساخت دریافت نشده است",
        "Build information not reported by this peer" => {
            "این دستگاه اطلاعات ساخت را گزارش نکرده است"
        }
        "Use the same live workspace controls; settings does not duplicate device editors." => {
            "از کنترل‌های زندهٔ فضای کاری استفاده کنید؛ ویرایشگرهای دستگاه در تنظیمات تکرار نمی‌شوند."
        }
        "Table & mapping" => "میز و نگاشت",
        "Recent colors" => "رنگ‌های اخیر",
        "Shared colors" => "رنگ‌های مشترک",
        "Choose color" => "انتخاب رنگ",
        "Text presentation" => "نمایش متن",
        "Buzzer & melodies" => "بیزر و ملودی‌ها",
        "Waiting for acknowledgment and saved-settings readback…" => {
            "در انتظار تأیید و بازخوانی تنظیمات ذخیره‌شده…"
        }
        "Settings changed on another client. Your edits are preserved; discard and reload before saving." => {
            "تنظیمات در کلاینت دیگری تغییر کرده‌اند. ویرایش‌های شما حفظ شده‌اند؛ پیش از ذخیره، تغییرات را لغو کنید و دوباره بخوانید."
        }
        "Unsaved device edits" => "ویرایش‌های ذخیره‌نشدهٔ دستگاه",
        "Observed device settings; no pending edits" => {
            "تنظیمات دستگاه دریافت شده‌اند؛ ویرایش ذخیره‌نشده‌ای نیست"
        }
        "Reconnect to read device settings" => "برای خواندن تنظیمات دستگاه دوباره متصل شوید",
        "Station SSID" => "نام شبکهٔ ایستگاه",
        "Station password · re-enter on save" => "گذرواژهٔ ایستگاه · هنگام ذخیره دوباره وارد کنید",
        "Open station network · no password" => "شبکهٔ باز ایستگاه · بدون گذرواژه",
        "Station enabled" => "ایستگاه فعال",
        "Recovery AP enabled" => "نقطهٔ دسترسی بازیابی فعال",
        "Scanned networks" => "شبکه‌های یافت‌شده",
        "The station password is write-only. Re-enter it to save Wi-Fi changes, or explicitly select an open network. Keep at least one network path enabled." => {
            "گذرواژهٔ ایستگاه فقط نوشتنی است. برای ذخیرهٔ تغییرات وای‌فای، آن را دوباره وارد کنید یا شبکهٔ باز را انتخاب کنید. دست‌کم یک مسیر شبکه را فعال نگه دارید."
        }
        "Enable MQTT" => "فعال‌کردن MQTT",
        "Broker hostname" => "نام میزبان بروکر",
        "Base topic" => "موضوع پایه",
        "Password handling" => "مدیریت گذرواژه",
        "Replacement password" => "گذرواژهٔ جایگزین",
        "Enable outbound WebSocket" => "فعال‌کردن وب‌سوکت خروجی",
        "Server hostname" => "نام میزبان سرور",
        "Path" => "مسیر",
        "Token handling" => "مدیریت توکن",
        "Replacement token" => "توکن جایگزین",
        "Telemetry interval · seconds" => "فاصلهٔ دورسنجی · ثانیه",
        "Keep stored password" => "حفظ گذرواژهٔ ذخیره‌شده",
        "Leave password unset" => "گذرواژه تعیین نشود",
        "Keep stored token" => "حفظ توکن ذخیره‌شده",
        "Leave token unset" => "توکن تعیین نشود",
        "Wait for the current operation to finish" => "منتظر پایان عملیات جاری بمانید",
        "Saved public settings verified by device readback. Secret values remain write-only." => {
            "تنظیمات عمومی با بازخوانی دستگاه تأیید شدند. مقادیر محرمانه فقط نوشتنی می‌مانند."
        }
        "No acknowledged workspace panels" => "پنل تأییدشده‌ای برای فضای کاری نیست",
        "Reset panel layout" => "بازنشانی چیدمان پنل‌ها",
        "Show" => "نمایش",
        "System" => "سیستم",
        "Dark" => "تیره",
        "Light" => "روشن",
        "Appearance…" => "ظاهر…",
        "Apply appearance" => "اعمال ظاهر",
        "Apply" => "اعمال",
        "Sign out" => "خروج از حساب",
        "Select all" => "انتخاب همه",
        "Manage cells…" => "مدیریت خانه‌ها…",
        "Toggle without Ctrl" => "انتخاب چندتایی بدون Ctrl",
        "Animate cells" => "پویانمایی خانه‌ها",
        "LED-state indicators" => "نشانگرهای وضعیت LED",
        "Edit order" => "ویرایش ترتیب",
        "Reset order" => "بازنشانی ترتیب",
        "Cell shape" => "شکل خانه",
        "Rounded" => "گوشه‌های گرد",
        "Circle · six pixels" => "دایره · شش پیکسل",
        "Hexagon · six pixels" => "شش‌ضلعی · شش پیکسل",
        "Physical pixel indices" => "شمارهٔ پیکسل‌های فیزیکی",
        "Board modes" => "حالت‌های برد",
        "Live device" => "دستگاه زنده",
        "Clients" => "کلاینت‌ها",
        "Station" => "ایستگاه",
        "Recovery AP" => "نقطهٔ دسترسی بازیابی",
        "Cloud WebSocket" => "وب‌سوکت ابری",
        "Cell identity" => "هویت خانه",
        "Cell action" => "عملیات خانه",
        "Addressable LEDs" => "LEDهای آدرس‌پذیر",
        "Save logical / physical mapping" => "ذخیرهٔ نگاشت منطقی و فیزیکی",
        "Stream rainbow" => "ارسال جریان رنگین‌کمان",
        "Reset game…" => "بازنشانی بازی…",
        "Transition" => "گذار",
        "Edit identity…" => "ویرایش هویت…",
        "Strip effect" => "جلوهٔ نوار نور",
        "Snake direction" => "جهت مار",
        "Live display" => "نمایشگر زنده",
        "Segment readback unavailable" => "بازخوانی نمایشگر در دسترس نیست",
        "Display state unavailable" => "وضعیت نمایشگر در دسترس نیست",
        "Show number" => "نمایش عدد",
        "Text" => "متن",
        "Front-panel text" => "متن پنل جلویی",
        "Scroll short text" => "پیمایش متن کوتاه",
        "Repeat" => "تکرار",
        "Present" | "Present text" => "نمایش متن",
        "Stop text" => "توقف متن",
        "Melodies" => "ملودی‌ها",
        "Hardware state unavailable" => "وضعیت سخت‌افزار در دسترس نیست",
        "Controllers" => "کنترل‌گرها",
        "Inputs" => "ورودی‌ها",
        "Shift-register outputs" => "خروجی‌های شیفت‌رجیستر",
        "Board identity" => "هویت برد",
        "Save board identity" => "ذخیرهٔ هویت برد",
        "Unsaved board identity changes" => "تغییرات هویت برد ذخیره نشده‌اند",
        "Discard name edits" => "دور انداختن تغییرات نام",
        "Device access" => "دسترسی به دستگاه",
        "Save device credentials" => "ذخیرهٔ اطلاعات ورود دستگاه",
        "Device configuration" => "پیکربندی دستگاه",
        "Live services" => "سرویس‌های زنده",
        "Target rate" => "نرخ هدف",
        "Clear events" => "پاک‌کردن رویدادها",
        "Error" => "خطا",
        "Reset the current game?" => "بازی کنونی بازنشانی شود؟",
        "The winner and current game state will be cleared." => {
            "برنده و وضعیت کنونی بازی پاک می‌شوند."
        }
        "Connect to controller" => "اتصال به کنترل‌گر",
        "Wi-Fi" => "وای‌فای",
        "USB serial" => "سریال USB",
        "Port" => "درگاه",
        "Commands" => "فرمان‌ها",
        "Mapping" => "نگاشت",
        "Restore workspace" => "بازیابی فضای کاری",
        "Pakar table" => "میز پاکار",
        "Logical" | "Logical cell" => "خانهٔ منطقی",
        "Physical" | "Physical cell" => "خانهٔ فیزیکی",
        "Save mapping" => "ذخیرهٔ نگاشت",
        "Save new" => "ذخیرهٔ جدید",
        "Replace with current layout" => "جایگزینی با چیدمان کنونی",
        "Clear cell selection" => "پاک‌کردن انتخاب خانه‌ها",
        "Previous selected cell (Shift+F3)" => "خانهٔ انتخاب‌شدهٔ قبلی (Shift+F3)",
        "Next selected cell (Enter / F3)" => "خانهٔ انتخاب‌شدهٔ بعدی (Enter / F3)",
        "Find cell: 12, 3–8  ·  Ctrl+F" => "یافتن خانه: 12, 3–8  ·  Ctrl+F",
        "Find cells: 12, 3–8" => "یافتن خانه‌ها: 12, 3–8",
        "Search device settings" => "جست‌وجوی تنظیمات دستگاه",
        "Toggle multiple cells and rows without Ctrl" => "انتخاب چند خانه و ردیف بدون Ctrl",
        "Animate cell state transitions" => "پویانمایی گذار وضعیت خانه‌ها",
        "Show cell LED-state indicators" => "نمایش نشانگر وضعیت LED خانه‌ها",
        "Fill lamp backgrounds with state color" => "رنگ پس‌زمینهٔ چراغ‌ها مطابق وضعیت",
        "Use the board's state colors. Live individual-pixel previews remain unchanged." => "از رنگ وضعیت‌های برد استفاده می‌کند؛ پیش‌نمایش زندهٔ پیکسل‌ها تغییری نمی‌کند.",
        "Show physical pixel indices (top first, CCW)" => {
            "نمایش شمارهٔ پیکسل‌های فیزیکی (از بالا، پادساعت‌گرد)"
        }
        "Disable for reduced motion. Cell geometry never changes." => {
            "برای کاهش حرکت غیرفعال کنید. هندسهٔ خانه‌ها هرگز تغییر نمی‌کند."
        }
        "Optional corner dots for white/red/off state. Hidden by default." => {
            "نقطه‌های گوشه برای وضعیت سفید، قرمز و خاموش؛ در حالت پیش‌فرض پنهان‌اند."
        }
        "Drag panel tabs to rearrange or detach them. Workspace profiles save the current layout." => {
            "زبانه‌های پنل را برای مرتب‌سازی یا جداکردن بکشید. نمایهٔ فضای کاری چیدمان کنونی را ذخیره می‌کند."
        }
        "Application" => "برنامه",
        "Device" => "دستگاه",
        "Device settings…" | "Device settings" => "تنظیمات دستگاه",
        "Connect…" => "اتصال…",
        "Cell mapping…" | "Cell mapping" => "نگاشت خانه‌ها",
        "Preferences…" => "تنظیمات…",
        "Table" => "میز",
        "Pixel diagnostics" => "عیب‌یابی پیکسل‌ها",
        "Pixel corrections" => "اصلاح پیکسل‌ها",
        "Effect controls" => "کنترل جلوه‌ها",
        "Hardware monitor" => "پایش سخت‌افزار",
        "Network & cloud" => "شبکه و ابر",
        "Events" => "رویدادها",
        "Notifications" => "اعلان‌ها",
        "Theme" => "پوسته",
        "Palette" => "پالت رنگ",
        "UI scale" => "مقیاس رابط",
        "Control density" => "تراکم کنترل‌ها",
        "Compact" => "فشرده",
        "Regular" => "معمولی",
        "Comfortable" => "راحت",
        "Spacious" => "باز",
        "Search preferences" => "جست‌وجوی تنظیمات",
        "Search settings" => "جست‌وجوی تنظیمات",
        "Device endpoint" => "نشانی دستگاه",
        "Connection defaults" => "پیش‌فرض‌های اتصال",
        "Automatically connect to a single discovered board" => "اتصال خودکار به تنها برد پیداشده",
        "Username" => "نام کاربری",
        "Password" => "گذرواژه",
        "USB serial port" => "درگاه سریال USB",
        "Single instance" => "فقط یک نمونهٔ برنامه",
        "Close to system tray" => "بستن به سینی سیستم",
        "Always on top" => "همیشه روی پنجره‌ها",
        "Open Preferences in a native window" => "بازکردن تنظیمات در پنجرهٔ بومی",
        "In-app toast banners" => "اعلان‌های داخل برنامه",
        "Windows notifications" => "اعلان‌های ویندوز",
        "Allow Windows notification sound" => "پخش صدای اعلان ویندوز",
        "Send test notification" => "ارسال اعلان آزمایشی",
        "Toggle fullscreen" => "تغییر حالت تمام‌صفحه",
        "Manage workspaces…" => "مدیریت فضاهای کاری…",
        "Reset workspace layout" => "بازنشانی چیدمان فضای کاری",
        "Auto-hide Events when errors clear" => "پنهان‌کردن خودکار رویدادها پس از رفع خطاها",
        "Save workspace screenshot…" => "ذخیرهٔ تصویر فضای کاری…",
        "Refresh" => "تازه‌سازی",
        "Find cell" => "یافتن خانه",
        "Manage cells" => "مدیریت خانه‌ها",
        "Cell number" => "شمارهٔ خانه",
        "Selected cells" => "خانه‌های انتخاب‌شده",
        "Device offline" => "دستگاه آفلاین",
        "Discard preference changes?" => "تغییرات تنظیمات دور انداخته شوند؟",
        "Your saved preferences will be restored." => "تنظیمات ذخیره‌شده بازیابی می‌شوند.",
        "Keep editing" => "ادامهٔ ویرایش",
        "Application icons" => "نمادهای برنامه",
        "Application icon" => "نماد برنامه",
        "Current" => "کنونی",
        "Classic (previous)" => "کلاسیک (قبلی)",
        "Default icon" => "نماد پیش‌فرض",
        "Bundled application icon" => "نماد همراه برنامه",
        "Use default icon" => "استفاده از نماد پیش‌فرض",
        "Browse..." => "مرور…",
        "Choose an image file" => "انتخاب پروندهٔ تصویر",
        "File" => "پرونده",
        "Edit" => "ویرایش",
        "Audio" => "صدا",
        "Subtitles" => "زیرنویس",
        "Active track" => "ترک فعال",
        "Hide subtitles" => "پنهان‌کردن زیرنویس",
        "Show subtitles" => "نمایش زیرنویس",
        "Workspace" => "فضای کاری",
        "Workspace profiles" => "نمایه‌های فضای کاری",
        "Workspace profiles..." => "نمایه‌های فضای کاری…",
        "Workspaces" => "فضاهای کاری",
        "Manage workspaces..." => "مدیریت فضاهای کاری…",
        "Manage workspaces" => "مدیریت فضاهای کاری",
        "Add workspace" => "افزودن فضای کاری",
        "No workspace profiles" => "هیچ فضای کاری وجود ندارد",
        "Active workspace" => "فضای کاری فعال",
        "Delete workspace" => "حذف فضای کاری",
        "Replace with current workspace" => "جایگزینی با فضای کاری کنونی",
        "Switch workspace" => "تغییر فضای کاری",
        "Save this window, panel, dialog, and scroll arrangement as a reusable workspace." => {
            "این چیدمان پنجره، پنل‌ها، گفتگوها و پیمایش را به‌عنوان فضای کاری قابل استفادهٔ مجدد ذخیره کنید."
        }
        "Workspace name" => "نام فضای کاری",
        "No saved workspace profiles" => "نمایهٔ فضای کاری ذخیره‌شده‌ای وجود ندارد",
        "Workspace profile saved" => "نمایهٔ فضای کاری ذخیره شد",
        "Workspace profile restored" => "نمایهٔ فضای کاری بازیابی شد",
        "Workspace profile deleted" => "نمایهٔ فضای کاری حذف شد",
        "Workspace profile not found" => "نمایهٔ فضای کاری پیدا نشد",
        "Enter a workspace profile name" => "نامی برای نمایهٔ فضای کاری وارد کنید",
        "Delete workspace profile" => "حذف نمایهٔ فضای کاری",
        "Restore" => "بازیابی",
        "Active" => "فعال",
        "Workspace tabs" => "زبانه‌های فضای کاری",
        "Right-click to show or hide workspace tabs" => {
            "برای نمایش یا پنهان‌کردن زبانه‌ها کلیک راست کنید"
        }
        "Window" => "پنجره",
        "Panels" => "پنل‌ها",
        "Reset Workspace to Default" => "بازنشانی فضای کاری به حالت پیش‌فرض",
        "All workspace panels are closed" => "همه پنل‌های فضای کاری بسته شده‌اند",
        "Open panels from the Workspace menu above, or reset the workspace." => {
            "پنل‌ها را از منوی فضای کاری در بالا باز کنید یا فضای کاری را بازنشانی نمایید."
        }
        "Help" => "راهنما",
        "Language" => "زبان",
        "Touch feedback" => "بازخورد لمسی",
        "System language" => "زبان سیستم",
        "English" => "انگلیسی",
        "Persian" => "فارسی",
        "Direction" => "جهت",
        "Automatic" => "خودکار",
        "Left to right" => "چپ به راست",
        "Right to left" => "راست به چپ",
        "Text alignment" => "تراز متن",
        "Left" => "چپ",
        "Center" => "وسط",
        "Right" => "راست",
        "Subtitle style" => "سبک زیرنویس",
        "Horizontal placement is independent of text direction" => {
            "جای افقی متن مستقل از جهت نوشتار است"
        }
        "Horizontal placement is independent of text direction; subtitle style preserves native styling without text processing" => {
            "جای افقی متن مستقل از جهت نوشتار است؛ سبک زیرنویس بدون پردازش متن حفظ می‌شود"
        }
        "Preserve subtitle styling without text processing; processed text stays centered" => {
            "حفظ سبک زیرنویس بدون پردازش متن؛ متن پردازش‌شده در وسط می‌ماند"
        }
        "Subtitle layout" => "چیدمان زیرنویس",
        "Global text replacements" => "جایگزینی‌های سراسری متن",
        "Applied in order to every text subtitle" => "به‌ترتیب روی همهٔ زیرنویس‌های متنی اعمال می‌شود",
        "Source text" => "متن مبدأ",
        "Replacement" => "جایگزین",
        "Add replacement" => "افزودن جایگزینی",
        "Remove replacement" => "حذف جایگزینی",
        "Open Video File..." => "باز کردن پروندهٔ ویدئو…",
        "Open Location / URL..." => "باز کردن نشانی یا پیوند…",
        "Open Recent" => "پرونده‌های اخیر",
        "No recent media" => "رسانهٔ اخیری وجود ندارد",
        "Clear Recent" => "پاک کردن فهرست اخیر",
        "Close Video" => "بستن ویدئو",
        "Open Timeline Project..." => "باز کردن پروژهٔ خط زمانی…",
        "Save Timeline (Sidecar)" => "ذخیرهٔ خط زمانی کنار رسانه",
        "Save Timeline As..." => "ذخیرهٔ خط زمانی با نام…",
        "Quit" => "خروج",
        "Undo" => "واگرد",
        "Redo" => "ازنو",
        "Audio Track" => "ترک صوتی",
        "None" => "هیچ‌کدام",
        "Audio Settings..." => "تنظیمات صدا…",
        "Audio Settings" => "تنظیمات صدا",
        "Subtitle Track" => "ترک زیرنویس",
        "Enable Subtitles" => "فعال‌سازی زیرنویس",
        "Subtitle Settings..." => "تنظیمات زیرنویس…",
        "Subtitle Settings" => "تنظیمات زیرنویس",
        "NLE Layout (Docked)" => "چیدمان تدوین",
        "Simple Player" => "پخش‌کنندهٔ ساده",
        "Keyboard Shortcuts..." => "میان‌برهای صفحه‌کلید…",
        "About" => "درباره",
        "Modern 4D Video & Haptic Player" => "پخش‌کنندهٔ نوین ویدئوی چهاربعدی و بازخورد لمسی",
        "High-performance media playback powered by libmpv2, glow OpenGL, and egui." => {
            "پخش پرسرعت رسانه با libmpv2،‏ OpenGL و egui."
        }
        "Featuring real-time 4D haptic timeline synchronization and multi-track relay control." => {
            "با همگام‌سازی بلادرنگ خط زمانی چهاربعدی و کنترل چندترکی رله‌ها."
        }
        "Connect" => "اتصال",
        "Connecting…" => "در حال اتصال…",
        "Reconnecting…" => "در حال اتصال دوباره…",
        "Trigger box" => "فعال‌سازی جعبه",
        "Copy box IDs" => "کپی شمارهٔ جعبه‌ها",
        "Select enabled" => "انتخاب فعال‌ها",
        "Disconnect" => "قطع اتصال",
        "Hardware Disconnected" => "سخت‌افزار قطع است",
        "Read-only: firmware must advertise board/boot identity and HTTP fencing" => {
            "فقط خواندنی: میان‌افزار باید هویت برد و راه‌اندازی و حفاظت HTTP را اعلام کند"
        }
        "PCController coordinator connected" => "هماهنگ‌کنندهٔ PCController متصل است",
        "Custom endpoint or hardware path" => "نشانی سفارشی یا مسیر سخت‌افزار",
        "Enter a PCController endpoint or an OS hardware path." => {
            "نشانی PCController یا مسیر سخت‌افزار سیستم‌عامل را وارد کنید."
        }
        "Enter a hardware endpoint before connecting." => {
            "پیش از اتصال، نشانی سخت‌افزار را وارد کنید."
        }
        "pccontroller://host:port, tcp://host:port, or direct:<device>" => {
            "pccontroller://میزبان:درگاه، tcp://میزبان:درگاه یا direct:<دستگاه>"
        }
        "E-STOP" => "توقف اضطراری",
        "E-STOP ACTIVE" => "توقف اضطراری فعال است",
        "Emergency stop" => "توقف اضطراری",
        "Hide E-STOP" => "پنهان‌کردن توقف اضطراری",
        "Release E-STOP?" => "توقف اضطراری آزاد شود؟",
        "Release E-STOP" => "آزادکردن توقف اضطراری",
        "Hardware outputs and effects will be allowed again." => {
            "خروجی‌های سخت‌افزار و جلوه‌ها دوباره مجاز خواهند شد."
        }
        "Do not ask again" => "دوباره پرسیده نشود",
        "Show E-STOP in the application header" => "نمایش توقف اضطراری در سربرگ برنامه",
        "Confirm before releasing E-STOP" => "تأیید پیش از آزادکردن توقف اضطراری",
        "Program Monitor" => "نمایش برنامه",
        "Media Inspector" => "بازرسی رسانه",
        "Refresh from libmpv" => "تازه‌سازی از libmpv",
        "Copy all" => "کپی همه",
        "Search properties" => "جست‌وجوی ویژگی‌ها",
        "No media loaded" => "هیچ رسانه‌ای بارگذاری نشده است",
        "Open media to inspect its libmpv properties." => {
            "برای بررسی ویژگی‌های libmpv یک رسانه باز کنید."
        }
        "Effect Controls" => "کنترل جلوه‌ها",
        "Identity" => "هویت",
        "Timing constraints" => "محدودیت‌های زمانی",
        "Start time:" => "زمان آغاز:",
        "Duration:" => "مدت:",
        "Hardware target" => "هدف سخت‌افزاری",
        "Connect PCController to choose an available output." => {
            "برای انتخاب خروجی در دسترس به PCController متصل شوید."
        }
        "Target output:" => "خروجی هدف:",
        "Bulk effect controls" => "کنترل گروهی جلوه‌ها",
        "Bulk hardware target override" => "تغییر گروهی هدف سخت‌افزاری",
        "Bulk actions" => "عملیات گروهی",
        "Tracks" => "ترک‌ها",
        "Effects Library" => "کتابخانهٔ جلوه‌ها",
        "Hardware Monitor" => "پایش سخت‌افزار",
        "Timeline" => "خط زمانی",
        "Hardware Monitor Dashboard" => "داشبورد پایش سخت‌افزار",
        "Hardware control" => "کنترل سخت‌افزار",
        "Manage..." => "مدیریت…",
        "Reveal in Effect Controls" => "نمایش در کنترل‌های جلوه",
        "Manage channels" => "مدیریت کانال‌ها",
        "All board channels" => "همهٔ کانال‌های برد",
        "Order" => "ترتیب",
        "Order must be between" => "ترتیب باید بین این مقادیر باشد:",
        "Move up" => "انتقال به بالا",
        "Move down" => "انتقال به پایین",
        "Other controls" => "کنترل‌های دیگر",
        "Rename, control, and reorder every advertised board channel" => {
            "تغییر نام، کنترل و مرتب‌سازی همهٔ کانال‌های اعلام‌شدهٔ برد"
        }
        "Presentation" => "نحوهٔ نمایش",
        "Save presentation" => "ذخیرهٔ نحوهٔ نمایش",
        "Restore defaults" => "بازگردانی پیش‌فرض‌ها",
        "Live control" => "کنترل زنده",
        "Turn on" => "روشن‌کردن",
        "Turn off" => "خاموش‌کردن",
        "Stable key" => "کلید پایدار",
        "This item has no live actions advertised." => {
            "برای این مورد هیچ عملیات زنده‌ای اعلام نشده است."
        }
        "Place at playhead" => "قرار دادن در نشانگر پخش",
        "The Controller effects track is not available." => "ترک جلوه‌های کنترل‌کننده در دسترس نیست.",
        "Manage" => "مدیریت",
        "Publish" => "انتشار",
        "Jump to cue start" => "پرش به آغاز نشانه",
        "Cue start" => "آغاز نشانه",
        "Run now" => "اجرای فوری",
        "Starting…" => "در حال آغاز…",
        "Stopping…" => "در حال توقف…",
        "Restore default name" => "بازگردانی نام پیش‌فرض",
        "Type" => "نوع",
        "Control" => "کنترل",
        "Board" => "برد",
        "Collapse group" => "جمع‌کردن گروه",
        "Expand group" => "بازکردن گروه",
        "New effect in this group" => "جلوهٔ جدید در این گروه",
        "New group" => "گروه جدید",
        "Color palette" => "پالت رنگ",
        "Studio" => "استودیو",
        "Neutral" => "خنثی",
        "Neutral (default)" => "خنثی (پیش‌فرض)",
        "Create effect" => "ایجاد جلوه",
        "Edit effect" => "ویرایش جلوه",
        "My effects" => "جلوه‌های من",
        "Macro" => "ماکرو",
        "Strip" => "نوار نور",
        "New" => "جدید",
        "Duplicate" => "تکثیر",
        "Description" => "توضیحات",
        "Manage effects" => "مدیریت جلوه‌ها",
        "Board information" => "اطلاعات برد",
        "Overview" => "نمای کلی",
        "Capabilities" => "قابلیت‌ها",
        "Front panel" => "پنل جلویی",
        "Board settings" => "تنظیمات برد",
        "Only settings advertised by the connected board are shown." => {
            "فقط تنظیماتی که برد متصل اعلام کرده است نمایش داده می‌شوند."
        }
        "General" => "عمومی",
        "Door sound cues" => "نشانه‌های صوتی در",
        "Relay sound cues" => "نشانه‌های صوتی رله",
        "Telemetry period" => "دورهٔ تله‌متری",
        "Use 0 to disable periodic telemetry" => {
            "برای غیرفعال‌کردن تله‌متری دوره‌ای مقدار ۰ را وارد کنید"
        }
        "Temperature sensors" => "حسگرهای دما",
        "Swap temperature roles" => "جابجایی نقش حسگرهای دما",
        "Enclosure lighting" => "روشنایی محفظه",
        "Automatic (door)" => "خودکار (در)",
        "Front-panel display" => "نمایشگر پنل جلویی",
        "Open brightness" => "روشنایی در حالت باز",
        "Closed brightness" => "روشنایی در حالت بسته",
        "Remember last page" => "به‌خاطر سپردن آخرین صفحه",
        "Status light" => "چراغ وضعیت",
        "Fallback color" => "رنگ جایگزین",
        "Red" => "قرمز",
        "Blue" => "آبی",
        "Violet" => "بنفش",
        "Green" => "سبز",
        "White" => "سفید",
        "Motion and relays" => "حرکت و رله‌ها",
        "Door policy" => "سیاست در",
        "Always allow motion" => "حرکت همیشه مجاز باشد",
        "Only while door is closed" => "فقط هنگامی که در بسته است",
        "Only while door is open" => "فقط هنگامی که در باز است",
        "Never allow motion" => "حرکت هرگز مجاز نباشد",
        "Exit hold" => "مکث خروج",
        "Relays restored after restart" => "رله‌های بازیابی‌شده پس از راه‌اندازی",
        "Remember motion defaults" => "به‌خاطر سپردن پیش‌فرض‌های حرکت",
        "Remember user relays" => "به‌خاطر سپردن رله‌های کاربر",
        "Remember PWM outputs" => "به‌خاطر سپردن خروجی‌های PWM",
        "Retain motion direction when stopped" => "حفظ جهت حرکت هنگام توقف",
        "Measurements" => "اندازه‌گیری‌ها",
        "Voltage decimals" => "رقم‌های اعشاری ولتاژ",
        "Current decimals" => "رقم‌های اعشاری جریان",
        "Advanced" => "پیشرفته",
        "Programming latch" => "قفل برنامه‌ریزی",
        "Programming latch blocks motion, relays, PWM, and lighting until disabled." => {
            "قفل برنامه‌ریزی تا زمان غیرفعال‌شدن، حرکت، رله‌ها، PWM و روشنایی را مسدود می‌کند."
        }
        "Save to board" => "ذخیره در برد",
        "Revert" => "بازگردانی",
        "Live settings" => "تنظیمات زنده",
        "Board name" => "نام برد",
        "Change icon" => "تغییر نماد",
        "Rename" => "تغییر نام",
        "Stored board name" => "نام ذخیره‌شدهٔ برد",
        "Product identity" => "هویت محصول",
        "Name storage" => "محل ذخیرهٔ نام",
        "EEPROM (persisted)" => "EEPROM (ذخیره‌شده)",
        "Not persisted" => "ذخیره نشده",
        "Not assigned" => "تعیین نشده",
        "Device path" => "مسیر دستگاه",
        "Device name" => "نام دستگاه",
        "Friendly name" => "نام خوانا",
        "Product" => "محصول",
        "Manufacturer" => "سازنده",
        "USB identity" => "شناسهٔ USB",
        "Serial number" => "شمارهٔ سریال",
        "Device instance" => "نمونهٔ دستگاه",
        "Rename board" => "تغییر نام برد",
        "Board kind" => "نوع برد",
        "Build hash" => "شناسهٔ ساخت",
        "Build timestamp" => "زمان ساخت",
        "Profile" => "پروفایل",
        "Profile revision" => "بازبینی پروفایل",
        "Mode" => "حالت",
        "Not advertised" => "اعلام نشده",
        "Stored by the attached board and read back through PCController." => {
            "در برد متصل ذخیره می‌شود و از طریق PCController بازخوانی می‌شود."
        }
        "Board commands" => "فرمان‌های برد",
        "Reboot board" => "راه‌اندازی دوبارهٔ برد",
        "Confirm board reboot" => "تأیید راه‌اندازی دوبارهٔ برد",
        "Advertised capabilities" => "قابلیت‌های اعلام‌شده",
        "PWM outputs" => "خروجی‌های PWM",
        "RF transmit" => "ارسال رادیویی",
        "Addressable strip" => "نوار نور آدرس‌پذیر",
        "Addressable LED strip" => "نوار LED آدرس‌پذیر",
        "Pixel count" => "تعداد پیکسل‌ها",
        "pixels" => "پیکسل",
        "Configure" => "پیکربندی",
        "Solid color" => "رنگ یکدست",
        "Single pixel" => "یک پیکسل",
        "Color frame" => "فریم رنگی",
        "Rainbow" => "رنگین‌کمان",
        "Color" => "رنگ",
        "Fill strip" => "پر کردن نوار",
        "Pixel" => "پیکسل",
        "Apply pixel" => "اعمال پیکسل",
        "Start color" => "رنگ آغاز",
        "End color" => "رنگ پایان",
        "Send color frame" => "ارسال فریم رنگی",
        "Frame rate" => "نرخ فریم",
        "Start rainbow" => "شروع رنگین‌کمان",
        "Select effect" => "انتخاب جلوه",
        "Start effect" => "شروع جلوه",
        "Clear strip" => "پاک کردن نوار",
        "Stop stream" => "توقف جریان",
        "Streaming" => "در حال پخش",
        "Idle" => "آماده",
        "Playing melody" => "ملودی در حال پخش است",
        "Playing" => "در حال پخش",
        "Board muted" => "برد بی‌صدا است",
        "Board audible" => "صدای برد فعال است",
        "Configured melody" => "ملودی پیکربندی‌شده",
        "No configured melodies" => "هیچ ملودی پیکربندی نشده است",
        "notes" => "نت",
        "Refresh melody catalog" => "تازه‌سازی فهرست ملودی‌ها",
        "Loop until stopped" => "تکرار تا زمان توقف",
        "Repeats" => "تعداد تکرار",
        "Play melody" => "پخش ملودی",
        "Stop buzzer" => "توقف بیزر",
        "Tone test" => "آزمایش صدا",
        "Frequency" => "فرکانس",
        "Play tone" => "پخش صدا",
        "The physical board is muted; host routing may still be audible." => {
            "برد فیزیکی بی‌صدا است؛ مسیر صدای میزبان ممکن است همچنان شنیده شود."
        }
        "Unavailable" => "در دسترس نیست",
        "No addressable LED effects are advertised" => "هیچ جلوهٔ LED آدرس‌پذیری اعلام نشده است",
        "Seven-segment display" => "نمایشگر هفت‌بخشی",
        "LCD display" => "نمایشگر LCD",
        "Board macros" => "ماکروهای برد",
        "Timed effects" => "جلوه‌های زمان‌بندی‌شده",
        "Capability bits" => "بیت‌های قابلیت",
        "Controls" => "کنترل‌ها",
        "Relays" => "رله‌ها",
        "PWM channels" => "کانال‌های PWM",
        "Peripherals" => "تجهیزات جانبی",
        "Macros" => "ماکروها",
        "Lighting effects" => "جلوه‌های نورپردازی",
        "Live front-panel state" => "وضعیت زندهٔ پنل جلویی",
        "Live physical display and board controls" => "نمایشگر فیزیکی زنده و کنترل‌های برد",
        "Live physical display" => "نمایشگر فیزیکی زنده",
        "Physical board keys" => "کلیدهای فیزیکی برد",
        "Read the exact state from the physical board" => "خواندن وضعیت دقیق از برد فیزیکی",
        "Reading the physical front panel" => "در حال خواندن پنل جلویی فیزیکی",
        "Pealayer is requesting the exact display frame from PCController." => {
            "Pealayer در حال دریافت فریم دقیق نمایشگر از PCController است."
        }
        "Physical front panel refreshed" => "پنل جلویی فیزیکی به‌روز شد",
        "Technical readback" => "خوانش فنی",
        "Display active" => "نمایشگر فعال",
        "Display idle" => "نمایشگر غیرفعال",
        "Schema" => "طرحواره",
        "Page" => "صفحه",
        "Previous" => "قبلی",
        "Next" => "بعدی",
        "Decrease" => "کاهش",
        "Increase" => "افزایش",
        "Select" => "انتخاب",
        "No exact seven-segment frame is available." => "فریم دقیق نمایشگر هفت‌بخشی در دسترس نیست.",
        "Send the same front-panel key press through PCController" => {
            "ارسال همان فشار کلید پنل جلویی از طریق PCController"
        }
        "The attached board does not advertise front-panel state." => {
            "برد متصل وضعیت پنل جلویی را اعلام نمی‌کند."
        }
        "The attached board does not advertise settings." => "برد متصل تنظیمات را اعلام نمی‌کند.",
        "Raw segments" => "بخش‌های خام",
        "Brightness" => "روشنایی",
        "Blink" => "چشمک",
        "Pressed keys" => "کلیدهای فشرده",
        "Menu page" => "صفحهٔ منو",
        "Program mode" => "حالت برنامه",
        "LCD available" => "LCD در دسترس",
        "LCD address" => "نشانی LCD",
        "LCD line 1" => "خط اول LCD",
        "LCD line 2" => "خط دوم LCD",
        "Silent mode" => "حالت بی‌صدا",
        "Light mode" => "حالت نور",
        "On brightness" => "روشنایی حالت روشن",
        "Off brightness" => "روشنایی حالت خاموش",
        "Display brightness" => "روشنایی نمایشگر",
        "Status brightness" => "روشنایی وضعیت",
        "Output persistence" => "ماندگاری خروجی",
        "Stream period" => "دورهٔ جریان",
        "Default page" => "صفحهٔ پیش‌فرض",
        "Motion break" => "وقفهٔ حرکت",
        "Persisted" => "ذخیره‌شده",
        "Yes" => "بله",
        "No" => "خیر",
        "Interface" => "رابط",
        "Position" => "موقعیت",
        "Player controls" => "کنترل‌های پخش‌کننده",
        "Playback speed" => "سرعت پخش",
        "Hold-to-fast-forward speed" => "سرعت جلورفتن با نگه‌داشتن",
        "Normal speed used for playback and restored after a temporary fast-forward" => {
            "سرعت عادی پخش که پس از جلورفتن موقت بازیابی می‌شود"
        }
        "Speed used while holding the configured temporary fast-forward gesture" => {
            "سرعت هنگام نگه‌داشتن حرکت جلورفتن موقت"
        }
        "Connection" => "اتصال",
        "Reconnect" => "اتصال دوباره",
        "Refresh live status" => "تازه‌سازی وضعیت زنده",
        "View options" => "گزینه‌های نمایش",
        "Compact controls" => "کنترل‌های فشرده",
        "Show raw relays" => "نمایش رله‌های خام",
        "Prefix relay identifiers" => "نمایش شناسهٔ رله پیش از نام",
        "Connection behavior" => "رفتار اتصال",
        "Connect automatically" => "اتصال خودکار",
        "Pause playback on disconnect" => "توقف پخش هنگام قطع اتصال",
        "Hardware endpoint" => "نشانی سخت‌افزار",
        "Connected" => "متصل",
        "Motion controls" => "کنترل‌های حرکتی",
        "Output controls" => "کنترل خروجی‌ها",
        "Update PWM outputs while dragging" => "به‌روزرسانی زندهٔ خروجی‌های PWM هنگام کشیدن",
        "Activate output buttons when pressed" => "فعال‌سازی دکمه‌های خروجی هنگام فشردن",
        "Motion / seat controls" => "کنترل حرکت / صندلی",
        "Moving up color" => "رنگ حرکت رو به بالا",
        "Moving down color" => "رنگ حرکت رو به پایین",
        "Button behavior" => "رفتار دکمه",
        "Hold" => "نگه‌داشتن",
        "Push" => "فشاری",
        "Toggle" => "تغییر حالت",
        "Move only while the button is held" => "حرکت فقط هنگام نگه‌داشتن دکمه",
        "Keep moving until Stop is pressed" => "ادامهٔ حرکت تا فشردن توقف",
        "Show raw relay controls" => "نمایش کنترل‌های رلهٔ خام",
        "Show raw relay controls below seat controls" => "نمایش کنترل‌های رلهٔ خام زیر کنترل صندلی",
        "Prefix relay captions with channel identifiers" => "نمایش شناسهٔ کانال پیش از نام رله",
        "Raw relays" => "رله‌های خام",
        "Raw relay" => "رلهٔ خام",
        "Relay" => "رله",
        "Board controls" => "کنترل‌های برد",
        "Board tools" => "ابزارهای برد",
        "Message for the board displays" => "پیام برای نمایشگرهای برد",
        "Segments" => "نمایشگر هفت‌بخشی",
        "Both displays" => "هر دو نمایشگر",
        "Bits" => "بیت‌ها",
        "Protocol" => "پروتکل",
        "Hex or decimal" => "هگز یا دهدهی",
        "Ready" => "آماده",
        "Box" => "جعبه",
        "Column" => "ستون",
        "Row" => "ردیف",
        "Button" => "دکمه",
        "Indicator" => "نشانگر",
        "Pressed" => "فشرده",
        "Winner" => "برنده",
        "Lost" => "بازنده",
        "Drag or use arrow keys; Esc cancels" => "بکشید یا از کلیدهای جهت استفاده کنید؛ لغو با Esc",
        "Live pixel preview not received" => "پیش‌نمایش زندهٔ پیکسل دریافت نشده است",
        "Invalid code" => "کد نامعتبر",
        "Transmit" => "ارسال",
        "State not sampled by the board" => "وضعیت توسط برد نمونه‌برداری نشده است",
        "Toggle on press" => "تغییر حالت با فشردن",
        "Run only while held" => "حرکت فقط هنگام نگه‌داشتن",
        "Use one-row compact hardware controls" => "استفاده از کنترل سخت‌افزار فشردهٔ یک‌ردیفه",
        "Use compact timeline track rows" => "استفاده از ردیف‌های فشرده در خط زمان",
        "Compact track rows" => "ردیف‌های فشردهٔ خط زمان",
        "Toggle mode keeps a direction active until another action is chosen. Hold mode sends Stop when the pressed direction is released." => {
            "در حالت تغییر وضعیت، جهت تا انتخاب فرمان بعدی فعال می‌ماند. در حالت نگه‌داشتن، با رهاکردن دکمه فرمان توقف فرستاده می‌شود."
        }
        "Video surface" => "سطح ویدئو",
        "Numeric input" => "ورودی عددی",
        "Human-readable time units" => "واحدهای زمانی خوانا",
        "Automatically normalize time fields to units such as s, min, and h. Disable to always display milliseconds." => {
            "نمایش خودکار زمان با واحدهای خوانا مانند ثانیه، دقیقه و ساعت؛ برای نمایش همیشگی میلی‌ثانیه غیرفعال کنید."
        }
        "Keyboard shortcuts" => "میان‌برهای صفحه‌کلید",
        "Enable in-app keyboard shortcuts and hardware bindings" => {
            "فعال‌سازی میان‌برها و کلیدهای سخت‌افزاری داخل برنامه"
        }
        "Leave a shortcut blank to disable it. Modified shortcuts remain available while editing text; bare printable keys do not interrupt typing." => {
            "برای غیرفعال‌کردن، میان‌بر را خالی بگذارید. میان‌برهای دارای کلید کمکی هنگام ویرایش متن نیز فعال‌اند؛ کلیدهای چاپ‌پذیرِ تنها مزاحم تایپ نمی‌شوند."
        }
        "Allow hardware hotkeys while Pealayer is in the background" => {
            "فعال‌سازی کلیدهای سراسری سخت‌افزار هنگامی که پی‌لیر در پس‌زمینه است"
        }
        "Keyboard, mouse, and gesture bindings" => "کلیدها، ماوس و حرکت‌های ورودی",
        "Left-button hold / drag while paused" => "نگه‌داشتن یا کشیدن دکمهٔ چپ هنگام مکث",
        "Left-button hold / drag while playing" => "نگه‌داشتن یا کشیدن دکمهٔ چپ هنگام پخش",
        "Middle-button click" => "کلیک دکمهٔ میانی",
        "Middle-button hold / drag" => "نگه‌داشتن یا کشیدن دکمهٔ میانی",
        "Right-button click" => "کلیک دکمهٔ راست",
        "Right-button hold / drag" => "نگه‌داشتن یا کشیدن دکمهٔ راست",
        "Play / pause" => "پخش یا مکث",
        "Mute / unmute" => "قطع یا وصل صدا",
        "Open context menu" => "باز کردن منوی زمینه‌ای",
        "Move application window" => "جابجایی پنجرهٔ برنامه",
        "Seek video" => "پیمایش ویدئو",
        "Temporarily fast-forward" => "تندپخش موقت",
        "Do nothing" => "بدون عملکرد",
        "Status bar" => "نوار وضعیت",
        "Hide" => "پنهان‌کردن",
        "Hide channel" => "پنهان‌کردن کانال",
        "Hide this channel from the Hardware Monitor" => "پنهان‌کردن این کانال از پایشگر سخت‌افزار",
        "Hidden channels" => "کانال‌های پنهان",
        "Show channel" => "نمایش کانال",
        "All channels are hidden" => "همهٔ کانال‌ها پنهان هستند",
        "hidden" => "پنهان",
        "Lock channel" => "قفل‌کردن کانال",
        "Channel is locked in PCController" => "کانال در PCController قفل است",
        "Prevent control changes until this channel is unlocked" => {
            "جلوگیری از تغییر کنترل تا زمانی که قفل کانال باز شود"
        }
        "Lock" => "قفل",
        "Prevent live control" => "جلوگیری از کنترل زنده",
        "Visibility" => "نمایانی",
        "Show in Hardware Monitor" => "نمایش در پایشگر سخت‌افزار",
        "Drag to reorder channel" => "برای تغییر ترتیب کانال بکشید",
        "Saving channel order..." => "در حال ذخیرهٔ ترتیب کانال‌ها…",
        "Channel order saved" => "ترتیب کانال‌ها ذخیره شد",
        "Unable to reorder these channels" => "تغییر ترتیب این کانال‌ها ممکن نیست",
        "Hardware connection" => "اتصال سخت‌افزار",
        "Hardware telemetry" => "دورسنجی سخت‌افزار",
        "Physical status RGB" => "چراغ RGB فیزیکی",
        "Hardware warnings" => "هشدارهای سخت‌افزار",
        "Clock" => "ساعت",
        "Media rate" => "نرخ رسانه",
        "Media frame rate" => "نرخ قاب رسانه",
        "Telemetry" => "دورسنجی",
        "Workspace mode" => "حالت فضای کاری",
        "Search" => "جست‌وجو",
        "Search effects..." => "جست‌وجوی جلوه‌ها…",
        "No effects" => "جلوه‌ای موجود نیست",
        "Until stopped" => "تا زمان توقف",
        "Loading hardware…" => "در حال بارگذاری سخت‌افزار…",
        "Board connection failed" => "اتصال برد ناموفق بود",
        "PCController detected hardware on" => "PCController سخت‌افزار را در این مسیر شناسایی کرد:",
        "but the board is not responding. Check its USB cable, power, and operating-system device status." => {
            "اما برد پاسخ نمی‌دهد. کابل USB، برق و وضعیت دستگاه در سیستم‌عامل را بررسی کنید."
        }
        "PCController is reachable, but no board is connected. Connect the board and check its USB cable, power, and port." => {
            "PCController در دسترس است، اما هیچ بردی متصل نیست. برد را متصل کنید و کابل USB، برق و درگاه آن را بررسی کنید."
        }
        "PCController connected; board not connected" => "PCController متصل است؛ برد متصل نیست",
        "The connected board advertises no relay controls." => {
            "برد متصل هیچ کنترل رله‌ای اعلام نکرده است."
        }
        "Connected board" => "برد متصل",
        "Group" => "گروه",
        "No group" => "بدون گروه",
        "Ungrouped" => "بدون گروه",
        "Search or create group..." => "جست‌وجو یا ایجاد گروه…",
        "Create group" => "ایجاد گروه",
        "New..." => "جدید…",
        "Select group" => "انتخاب گروه",
        "Change group" => "تغییر گروه",
        "Connected board:" => "برد متصل:",
        "Force ON" => "روشن‌کردن اجباری",
        "ON" => "روشن",
        "OFF" => "خاموش",
        "Release" => "رهاسازی",
        "Override requested" => "درخواست بازنویسی ارسال شد",
        "Board reports ON" => "برد حالت روشن را گزارش می‌کند",
        "Board reports OFF" => "برد حالت خاموش را گزارش می‌کند",
        "Live board control" => "کنترل زندهٔ برد",
        "PCController macro catalog" => "فهرست ماکروهای PCController",
        "Record hardware effect" => "ضبط جلوهٔ سخت‌افزاری",
        "Record effect" => "ضبط جلوه",
        "Seat motion take" => "برداشت حرکت صندلی",
        "Start board recording" => "آغاز ضبط از برد",
        "Start recording" => "آغاز ضبط",
        "Record from the current media time" => "ضبط از زمان کنونی رسانه",
        "All applied hardware actions" => "همهٔ فرمان‌های اعمال‌شدهٔ سخت‌افزار",
        "Relay snapshots in board RAM" => "نمونه‌های رله در حافظهٔ برد",
        "Refresh status" => "تازه‌سازی وضعیت",
        "Save and place" => "ذخیره و جای‌گذاری",
        "Discard" => "دور انداختن",
        "Timeline anchor:" => "نقطهٔ خط زمانی:",
        "Addressable strip effects" => "جلوه‌های نوار نور آدرس‌پذیر",
        "Stop preview" => "توقف پیش‌نمایش",
        "Preview" => "پیش‌نمایش",
        "No compatible relay effects are advertised by the connected PCController." => {
            "PCController متصل هیچ جلوهٔ رله‌ای سازگاری اعلام نکرده است."
        }
        "Connection problem" => "مشکل اتصال",
        "Playback problem" => "مشکل پخش",
        "Operation failed" => "عملیات ناموفق بود",
        "Pealayer could not complete the requested operation." => {
            "پی‌لیر نتوانست عملیات درخواستی را کامل کند."
        }
        "Technical details" => "جزئیات فنی",
        "Copy details" => "رونوشت جزئیات",
        "Close" => "بستن",
        "Save" => "ذخیره",
        "Preferences" => "تنظیمات",
        "Preferences saved" => "تنظیمات ذخیره شد",
        "Unsaved changes" => "تغییرات ذخیره‌نشده",
        "Unsaved preferences" => "تنظیمات ذخیره‌نشده",
        "Save your changes before closing Preferences?" => {
            "پیش از بستن تنظیمات، تغییرات ذخیره شوند؟"
        }
        "Preferences reloaded from disk" => "تنظیمات از دیسک بازخوانی شد",
        "No changes to save" => "تغییری برای ذخیره وجود ندارد",
        "Config file" => "پروندهٔ پیکربندی",
        "Reload from disk" => "بازخوانی از دیسک",
        "Edit config file" => "ویرایش پروندهٔ پیکربندی",
        "Edit config file in an external editor" => "ویرایش پروندهٔ پیکربندی در ویرایشگر بیرونی",
        "Open containing folder" => "باز کردن پوشهٔ پرونده",
        "Copy full path" => "رونوشت مسیر کامل",
        "Configuration path copied" => "مسیر کامل پیکربندی رونویسی شد",
        "Import" => "درون‌ریزی",
        "Export" => "برون‌ریزی",
        "Configuration imported; review and save" => {
            "پیکربندی درون‌ریزی شد؛ آن را بررسی و ذخیره کنید"
        }
        "Configuration exported" => "پیکربندی برون‌ریزی شد",
        "Automatically reload configuration changes" => "بازخوانی خودکار تغییرات پیکربندی",
        "Opened configuration in the external editor" => {
            "پروندهٔ پیکربندی در ویرایشگر بیرونی باز شد"
        }
        "Opened the configuration folder" => "پوشهٔ پیکربندی باز شد",
        "Configuration changed on disk; reload it or save your draft" => {
            "پیکربندی روی دیسک تغییر کرده است؛ آن را بازخوانی یا پیش‌نویس را ذخیره کنید"
        }
        "Open Location / URL" => "باز کردن نشانی یا پیوند",
        "Playback history" => "تاریخچهٔ پخش",
        "Remember the last position of local and remote media" => {
            "به‌خاطر سپردن آخرین موقعیت رسانه‌های محلی و راه‌دور"
        }
        "Maximum remembered videos" => "بیشترین تعداد ویدئوهای به‌خاطر سپرده‌شده",
        "Clear remembered positions" => "پاک‌کردن موقعیت‌های به‌خاطر سپرده‌شده",
        "Resumed at" => "ادامه از",
        "Enter a remote media location" => "نشانی رسانهٔ راه‌دور را وارد کنید",
        "Enter a media URL (HTTP/HTTPS, HLS, RTSP, RTMP, SRT, UDP, or TCP):" => {
            "نشانی رسانه (HTTP/HTTPS،‏ HLS،‏ RTSP،‏ RTMP،‏ SRT،‏ UDP یا TCP) را وارد کنید:"
        }
        "Enter direct video URL, HTTP/HTTPS stream, or HLS link:" => {
            "نشانی مستقیم ویدئو، جریان HTTP/HTTPS یا پیوند HLS را وارد کنید:"
        }
        "Wrap long URLs in a text area" => "پیوندهای بلند در کادر متنی چندخطی شکسته شوند",
        "Supported: HTTP/HTTPS, HLS, RTSP, RTMP, SRT, UDP, TCP, and RIST." => {
            "پشتیبانی‌شده: HTTP/HTTPS،‏ HLS،‏ RTSP،‏ RTMP،‏ SRT،‏ UDP،‏ TCP و RIST."
        }
        "Use proxy" => "استفاده از پراکسی",
        "Configure proxy" => "پیکربندی پراکسی",
        "Recent locations" => "نشانی‌های اخیر",
        "Clear all" => "پاک‌کردن همه",
        "No recent remote locations." => "نشانی راه‌دور اخیری وجود ندارد.",
        "Remove from history" => "حذف از تاریخچه",
        "Expand recent URL history by default" => "تاریخچهٔ نشانی‌های اخیر به‌طور پیش‌فرض باز باشد",
        "Fetch remote media information automatically" => {
            "اطلاعات رسانهٔ راه‌دور به‌طور خودکار دریافت شود"
        }
        "Fetch a remote media thumbnail automatically" => {
            "تصویر بندانگشتی رسانهٔ راه‌دور به‌طور خودکار دریافت شود"
        }
        "Use a proxy for URL inspection" => "استفاده از پراکسی برای بررسی نشانی",
        "Use a proxy for remote inspection and playback" => {
            "استفاده از پراکسی برای بررسی و پخش رسانهٔ راه‌دور"
        }
        "Inherited proxy" => "پراکسی به‌ارث‌رسیده",
        "Custom proxy URL" => "نشانی پراکسی سفارشی",
        "Enter a complete HTTP or HTTPS proxy URL." => {
            "یک نشانی کامل HTTP یا HTTPS برای پراکسی وارد کنید."
        }
        "Leave the custom proxy blank to inherit the operating-system environment." => {
            "برای استفاده از تنظیمات محیط سیستم‌عامل، نشانی پراکسی سفارشی را خالی بگذارید."
        }
        "MPV playback proxy" => "پراکسی پخش MPV",
        "MPV requires an http:// proxy URL; URL inspection can still use HTTPS proxy URLs." => {
            "MPV به نشانی پراکسی http:// نیاز دارد؛ بررسی نشانی همچنان می‌تواند از پراکسی HTTPS استفاده کند."
        }
        "MPV applies this proxy to supported HTTP media requests. HTTPS and extractor proxy support depends on the bundled MPV and FFmpeg backends." => {
            "MPV این پراکسی را برای درخواست‌های HTTP پشتیبانی‌شده به‌کار می‌برد. پشتیبانی پراکسی HTTPS و استخراج‌کننده به نسخه‌های همراه MPV و FFmpeg بستگی دارد."
        }
        "File associations" => "وابستگی‌های پرونده",
        "Registered media types" => "گونه‌های رسانهٔ ثبت‌شده",
        "Register Pealayer with the operating system, then choose it as the default app for the media types you want." => {
            "Pealayer را در سیستم‌عامل ثبت کنید، سپس آن را برای گونه‌های رسانهٔ دلخواه به‌عنوان برنامهٔ پیش‌فرض برگزینید."
        }
        "Register as a media player" => "ثبت به‌عنوان پخش‌کنندهٔ رسانه",
        "Remove file associations" => "حذف وابستگی‌های پرونده",
        "Removed media types" => "گونه‌های رسانهٔ حذف‌شده",
        "Clear remote history" => "پاک‌کردن تاریخچهٔ راه‌دور",
        "Clear from history" => "پاک‌کردن از تاریخچه",
        "Play when a recent location is clicked" => "با کلیک روی نشانی اخیر، آن را پخش کن",
        "Inspect now" => "بررسی اکنون",
        "Fetching details…" => "در حال دریافت جزئیات…",
        "Refresh details" => "تازه‌سازی جزئیات",
        "Retry details" => "تلاش دوباره برای جزئیات",
        "Fetch details" => "دریافت جزئیات",
        "Fetching details" => "در حال دریافت جزئیات",
        "Preparing remote media details…" => "در حال آماده‌سازی جزئیات رسانهٔ راه‌دور…",
        "Automatic remote information is disabled." => {
            "دریافت خودکار اطلاعات رسانهٔ راه‌دور غیرفعال است."
        }
        "Invalid media location" => "نشانی رسانه نامعتبر است",
        "Problem" => "مشکل",
        "Entered value" => "مقدار واردشده",
        "How to fix" => "راه‌حل",
        "Enter a complete supported URL including its protocol." => {
            "یک نشانی کامل و پشتیبانی‌شده همراه با پروتکل آن وارد کنید."
        }
        "Could not fetch remote media details" => "جزئیات رسانهٔ راه‌دور دریافت نشد",
        "Target" => "مقصد",
        "Reason" => "دلیل",
        "System default" => "پیش‌فرض سیستم",
        "Disabled" => "غیرفعال",
        "Proxy" => "پراکسی",
        "Next step" => "گام بعدی",
        "Check the address or connection, then retry the details request." => {
            "نشانی یا اتصال را بررسی کنید و سپس دریافت جزئیات را دوباره امتحان کنید."
        }
        "Cut" => "برش",
        "Copy" => "رونوشت",
        "Select All" => "انتخاب همه",
        "Valid media location" => "نشانی معتبر رسانه",
        "Remote media information" => "اطلاعات رسانهٔ راه‌دور",
        "Enter a location to validate and inspect it." => {
            "برای اعتبارسنجی و بررسی، یک نشانی وارد کنید."
        }
        "Waiting for a valid HTTP or HTTPS URL…" => "در انتظار یک نشانی معتبر HTTP یا HTTPS…",
        "Checking" => "در حال بررسی",
        "Status" => "وضعیت",
        "Final URL" => "نشانی نهایی",
        "File name" => "نام پرونده",
        "Content type" => "نوع محتوا",
        "Remote size" => "اندازهٔ راه‌دور",
        "Byte ranges" => "بازه‌های بایتی",
        "Last modified" => "آخرین تغییر",
        "Server" => "سرور",
        "Response time" => "زمان پاسخ",
        "Thumbnail at 20%" => "تصویر بندانگشتی در ۲۰٪",
        "Initial frame" => "نخستین قاب",
        "Thumbnail unavailable" => "تصویر بندانگشتی در دسترس نیست",
        "Paste" => "چسباندن",
        "Open" => "باز کردن",
        "Cancel" => "انصراف",
        "Keyboard Shortcuts & Controls" => "میان‌برها و کنترل‌های صفحه‌کلید",
        "Shortcut" => "میان‌بر",
        "Action" => "عملکرد",
        "Play / Pause video" => "پخش یا مکث ویدئو",
        "Toggle Fullscreen mode" => "تغییر حالت تمام‌صفحه",
        "Toggle Audio Mute" => "قطع یا وصل صدا",
        "Seek -5s / +5s" => "پیمایش ۵ ثانیه عقب یا جلو",
        "Volume -5% / +5%" => "کاهش یا افزایش ۵ درصدی صدا",
        "Frame Step Forward (+1 frame)" => "یک قاب جلو",
        "Frame Step Backward (-1 frame)" => "یک قاب عقب",
        "Adjust Volume on player/bar" => "تنظیم صدا روی پخش‌کننده",
        "Seek forward / backward" => "پیمایش جلو یا عقب",
        "Add cue to selected track" => "افزودن نشانه به ترک انتخاب‌شده",
        "Toggle Fullscreen / Open Video" => "تمام‌صفحه یا باز کردن ویدئو",
        "Open Player Context Menu" => "باز کردن منوی زمینه‌ای پخش‌کننده",
        "Drop media file onto window to play" => "رسانه را برای پخش روی پنجره رها کنید",
        "Track:" => "ترک:",
        "Render the selected subtitle track over the video" => {
            "نمایش ترک زیرنویس انتخاب‌شده روی ویدئو"
        }
        "Enabled" => "فعال",
        "Subtitle track" => "ترک زیرنویس",
        "Choose an embedded or externally loaded track" => {
            "انتخاب ترک داخلی یا بارگذاری‌شده از بیرون"
        }
        "Appearance and placement" => "ظاهر و جای‌گذاری",
        "Text direction" => "جهت متن",
        "Automatic, left-to-right, or right-to-left layout" => {
            "چیدمان خودکار، چپ‌به‌راست یا راست‌به‌چپ"
        }
        "Font size" => "اندازهٔ قلم",
        "Scale subtitle text without changing the video" => {
            "تغییر اندازهٔ متن زیرنویس بدون تغییر ویدئو"
        }
        "Location offset" => "جابجایی مکان",
        "0% places subtitles at the top; 100% places them at the bottom" => {
            "۰٪ زیرنویس را بالا و ۱۰۰٪ آن را پایین قرار می‌دهد"
        }
        "Subtitle delay" => "تأخیر زیرنویس",
        "Use negative values when subtitles appear too late" => {
            "اگر زیرنویس دیر نمایش داده می‌شود، از مقدار منفی استفاده کنید"
        }
        "Reset timing" => "بازنشانی زمان‌بندی",
        "External subtitle" => "زیرنویس بیرونی",
        "Attach an SRT, VTT, ASS, or SSA file to the current media" => {
            "افزودن فایل SRT، VTT، ASS یا SSA به رسانهٔ فعلی"
        }
        "Add subtitle file..." => "افزودن فایل زیرنویس…",
        "Audio track" => "ترک صدا",
        "Choose an embedded or externally loaded audio stream" => {
            "انتخاب جریان صدای داخلی یا بارگذاری‌شده از بیرون"
        }
        "Sound" => "صدا",
        "Temporarily silence playback without changing volume" => {
            "بی‌صدا کردن موقت پخش بدون تغییر بلندی صدا"
        }
        "Adjust playback volume from 0% to 130%" => "تنظیم بلندی صدای پخش از ۰٪ تا ۱۳۰٪",
        "Audio delay" => "تأخیر صدا",
        "Use negative values when audio is heard too late" => {
            "اگر صدا دیر شنیده می‌شود، از مقدار منفی استفاده کنید"
        }
        "External audio" => "صدای بیرونی",
        "Attach an audio file to the current media" => "افزودن فایل صدا به رسانهٔ فعلی",
        "Add audio file..." => "افزودن فایل صدا…",
        "Appearance" => "ظاهر",
        "Group Preferences navigation" => "گروه‌بندی پیمایش تنظیمات",
        "Personalization" => "شخصی‌سازی",
        "Connections" => "اتصال‌ها",
        "Maintenance" => "نگهداری",
        "Accent color" => "رنگ تأکیدی",
        "System accent" => "رنگ تأکیدی سامانه",
        "Pealayer green" => "سبز پی‌لیر",
        "Windows blue" => "آبی ویندوز",
        "macOS blue" => "آبی مک‌اواس",
        "Custom" => "سفارشی",
        "Custom accent" => "رنگ تأکیدی سفارشی",
        "Font Size:" => "اندازهٔ قلم:",
        "Synchronization" => "همگام‌سازی",
        "Delay (s):" => "تأخیر (ثانیه):",
        "Reset" => "بازنشانی",
        "Load External Subtitle..." => "بارگیری زیرنویس بیرونی…",
        "Load External Audio..." => "بارگیری صدای بیرونی…",
        "Workspace: NLE Layout" => "فضای کاری: تدوین",
        "Workspace: Simple Player" => "فضای کاری: پخش‌کنندهٔ ساده",
        "4D Cinema Editor" => "ویرایشگر سینمای چهاربعدی",
        "Active output status" => "وضعیت خروجی‌های فعال",
        "Effect templates" => "الگوهای جلوه",
        "Select a cue to inspect and edit" => "برای بررسی و ویرایش، یک نشانه را انتخاب کنید",
        "Timing, routing, and output" => "زمان‌بندی، مسیریابی و خروجی",
        "Batch-edit selected cues" => "ویرایش گروهی نشانه‌های انتخاب‌شده",
        "Icon" => "نماد",
        "Search icons..." => "جست‌وجوی نمادها…",
        "List view" => "نمای فهرست",
        "Grid view" => "نمای شبکه‌ای",
        "Presets" => "گزینه‌های آماده",
        "No matching icons" => "نماد منطبقی پیدا نشد",
        "Name" => "نام",
        "Duration" => "مدت",
        "Add cue" => "افزودن نشانه",
        "Cue added" => "نشانه افزوده شد",
        "On" => "روشن",
        "Off" => "خاموش",
        "Actions" => "عملیات",
        "Selected cue" => "نشانهٔ انتخاب‌شده",
        "1 cue" => "۱ نشانه",
        "cues" => "نشانه",
        "selected" => "انتخاب‌شده",
        "controller-owned" => "تحت مدیریت کنترل‌گر",
        "Multiple cues selected" => "چند نشانه انتخاب شده است",
        "Changes apply to every compatible cue" => "تغییرات روی همهٔ نشانه‌های سازگار اعمال می‌شود",
        "Timing" => "زمان‌بندی",
        "Exact timeline placement and length" => "جای‌گذاری و مدت دقیق در خط زمانی",
        "Starts" => "آغاز",
        "Go to cue" => "رفتن به نشانه",
        "Duplicate cue" => "تکثیر نشانه",
        "Untitled effect" => "جلوهٔ بی‌نام",
        "No cue selected" => "هیچ نشانه‌ای انتخاب نشده است",
        "Select a cue on the timeline to manage it" => {
            "برای مدیریت، نشانه‌ای را در خط زمانی انتخاب کنید"
        }
        "Select a hardware track and press A, or double-click its lane, to add a cue." => {
            "برای افزودن نشانه، یک ترک سخت‌افزاری را انتخاب و A را فشار دهید، یا روی مسیر آن دوبار کلیک کنید."
        }
        "Cue unavailable" => "نشانه در دسترس نیست",
        "The selected cue is no longer on the timeline" => "نشانهٔ انتخاب‌شده دیگر در خط زمانی نیست",
        "Source" => "منبع",
        "Addressable lighting" => "نورپردازی آدرس‌پذیر",
        "Hardware macro" => "ماکروی سخت‌افزاری",
        "PCController effect" => "جلوهٔ PCController",
        "Relay sequence" => "توالی رله",
        "Direct channel cue" => "نشانهٔ مستقیم کانال",
        "Timeline effect" => "جلوهٔ خط زمانی",
        "Live output on" => "خروجی زنده روشن است",
        "Live output off" => "خروجی زنده خاموش است",
        "No live hardware outputs" => "هیچ خروجی سخت‌افزاری زنده‌ای در دسترس نیست",
        "Output" => "خروجی",
        "Set hardware target" => "تعیین مقصد سخت‌افزاری",
        "Target mismatch" => "ناهماهنگی مقصد",
        "Configured output" => "خروجی پیکربندی‌شده",
        "Move cue to matching track" => "انتقال نشانه به ترک سازگار",
        "Add to timeline" => "افزودن به خط زمانی",
        "Create custom template" => "ساخت الگوی سفارشی",
        "Name:" => "نام:",
        "Icon:" => "نماد:",
        "Duration (ms):" => "مدت (میلی‌ثانیه):",
        "Output:" => "خروجی:",
        "No advertised output" => "هیچ خروجی‌ای اعلام نشده است",
        "Create" => "ساختن",
        "Timeline instances" => "نمونه‌های خط زمانی",
        "Start time" => "زمان آغاز",
        "Select a relay or PWM timeline track first" => {
            "ابتدا یک ترک رله یا PWM را در خط زمانی انتخاب کنید"
        }
        "Effect" => "جلوه",
        "Unavailable project effect" => "جلوهٔ پروژه در دسترس نیست",
        "Delete" => "حذف",
        "No Cue Selected" => "هیچ نشانه‌ای انتخاب نشده است",
        "Open media or connect PCController to populate the timeline." => {
            "برای ساخت خط زمانی، رسانه‌ای باز کنید یا به PCController متصل شوید."
        }
        "No media or advertised hardware tracks are available." => {
            "هیچ رسانه یا ترک سخت‌افزاری اعلام‌شده‌ای در دسترس نیست."
        }
        "Replay" => "پخش دوباره",
        "Play" => "پخش",
        "Pause" => "مکث",
        "Stop" => "توقف",
        "Fullscreen" => "تمام‌صفحه",
        "LIVE" => "زنده",
        "Determining duration…" => "در حال تعیین مدت…",
        "Open media to see its duration." => "برای دیدن مدت، رسانه‌ای باز کنید.",
        "MPV is still reading media metadata. Duration and seeking will update when available." => {
            "MPV هنوز در حال خواندن فرادادهٔ رسانه است. مدت و امکان جابه‌جایی پس از آماده‌شدن به‌روز می‌شوند."
        }
        "This live or duration-less source has no fixed endpoint or seek range." => {
            "این منبع زنده یا بدون مدت، نقطهٔ پایان یا بازهٔ جابه‌جایی ثابتی ندارد."
        }
        "Showing time remaining. Click to show total duration." => {
            "زمان باقی‌مانده نمایش داده می‌شود. برای نمایش مدت کل کلیک کنید."
        }
        "Showing total duration. Click to show time remaining." => {
            "مدت کل نمایش داده می‌شود. برای نمایش زمان باقی‌مانده کلیک کنید."
        }
        "Open media to seek." => "برای جابه‌جایی، رسانه‌ای باز کنید.",
        "Duration is still being determined; seeking will become available when MPV reports a timeline." => {
            "مدت هنوز در حال تعیین است؛ وقتی MPV خط زمانی را گزارش کند جابه‌جایی فعال می‌شود."
        }
        "This live or duration-less source has no fixed seek range." => {
            "این منبع زنده یا بدون مدت، بازهٔ جابه‌جایی ثابتی ندارد."
        }
        "This media reports a duration but does not support seeking." => {
            "این رسانه مدت را گزارش می‌کند اما از جابه‌جایی پشتیبانی نمی‌کند."
        }
        "Seek through the media timeline." => "در خط زمانی رسانه جابه‌جا شوید.",
        "Exit Fullscreen" => "خروج از تمام‌صفحه",
        "Controls Pinned" => "کنترل‌ها ثابت شدند",
        "Controls Unpinned" => "کنترل‌ها از حالت ثابت خارج شدند",
        "Pin Controls" => "ثابت کردن کنترل‌ها",
        "Unpin Controls" => "آزاد کردن کنترل‌ها",
        "Mute" => "قطع صدا",
        "Mute audio" => "قطع صدا",
        "Mute playback without changing the selected audio track." => {
            "صدا را بدون تغییر ترک صوتی انتخاب‌شده قطع می‌کند."
        }
        "Track" => "ترک",
        "Video disabled" => "تصویر غیرفعال",
        "No audio track" => "بدون ترک صوتی",
        "Subtitles hidden" => "زیرنویس پنهان",
        "No tracks available" => "ترکی موجود نیست",
        "Unmute" => "وصل صدا",
        "Video Files" => "پرونده‌های ویدئویی",
        "Unknown" => "ناشناخته",
        "Drop video file here to play" => "پروندهٔ ویدئو را برای پخش اینجا رها کنید",
        "Invalid format. Use HH:MM:SS.cs (e.g., 00:01:23.45)" => {
            "قالب نامعتبر است. از HH:MM:SS.cs استفاده کنید (برای نمونه 00:01:23.45)"
        }
        "Set to current playback position" => "تنظیم روی موقعیت کنونی پخش",
        "Seek video to this event" => "رفتن به زمان این رویداد در ویدئو",
        "Nudge back by" => "جابجایی به عقب به‌اندازهٔ",
        "Nudge forward by" => "جابجایی به جلو به‌اندازهٔ",
        "Hold Shift for 100ms, Ctrl for 1s" => "Shift برای ۱۰۰ms و Ctrl برای ۱s",
        "Dur" => "مدت",
        "End" => "پایان",
        "Unavailable output" => "خروجی در دسترس نیست",
        "Unavailable project output" => "خروجی پروژه در دسترس نیست",
        "Delete Cue" => "حذف نشانه",
        "Delete All Selected" => "حذف همهٔ انتخاب‌شده‌ها",
        "EMERGENCY STOP ACTIVE - ALL OUTPUTS DISABLED" => {
            "توقف اضطراری فعال است — همهٔ خروجی‌ها غیرفعال‌اند"
        }
        "Linear" => "خطی",
        "Smooth (Hermite)" => "نرم (هرمیت)",
        "Step" => "پله‌ای",
        "Delete Keyframe" => "حذف فریم کلیدی",
        "Video" => "ویدئو",
        "Volume" => "صدا",
        "Interpolation" => "درون‌یابی",
        "Time" => "زمان",
        "Value" => "مقدار",
        "Analog Track" => "ترک آنالوگ",
        "Port/Channel" => "درگاه/کانال",
        "Mute Track (M)\nMutes physical output during playback." => {
            "قطع ترک (M)\nخروجی فیزیکی را هنگام پخش غیرفعال می‌کند."
        }
        "Mute Track (M)\nMutes relay physical output during playback." => {
            "قطع ترک (M)\nخروجی فیزیکی رله را هنگام پخش غیرفعال می‌کند."
        }
        "Mute Track (M)\nMutes actuator physical output during playback." => {
            "قطع ترک (M)\nخروجی فیزیکی محرک را هنگام پخش غیرفعال می‌کند."
        }
        "Solo Track (S)\nSolos this relay track output during playback." => {
            "تک‌نوازی ترک (S)\nفقط خروجی این رله را هنگام پخش فعال نگه می‌دارد."
        }
        "Solo Track (S)\nSolos this output during playback." => {
            "تک‌نوازی ترک (S)\nفقط این خروجی را هنگام پخش فعال نگه می‌دارد."
        }
        "Lock Track (L)\nPrevents moving or modifying effects on this track." => {
            "قفل ترک (L)\nجابجایی یا ویرایش جلوه‌های این ترک را متوقف می‌کند."
        }
        "Record Arm (R)\nArms this track for real-time motion capture gesture recording." => {
            "آماده‌سازی ضبط (R)\nاین ترک را برای ضبط بلادرنگ حرکت آماده می‌کند."
        }
        "Live Actuator Fader\nControl actuator intensity in real time (0% - 100%)." => {
            "فیدر زندهٔ محرک\nشدت محرک را بلادرنگ کنترل می‌کند (۰ تا ۱۰۰٪)."
        }
        "Add Keyframe\nInserts a keyframe at the current playhead position." => {
            "افزودن فریم کلیدی\nیک فریم کلیدی در موقعیت کنونی نشانگر درج می‌کند."
        }
        "Use Ctrl or Command plus wheel to scroll tracks vertically" => {
            "پیمایش عمودی ترک‌ها با Ctrl یا Command و چرخ ماوس"
        }
        "Takes priority over Ctrl or Command plus wheel zoom; Shift plus wheel still scrolls horizontally" => {
            "بر بزرگ‌نمایی با Ctrl یا Command اولویت دارد؛ Shift و چرخ ماوس همچنان پیمایش افقی است"
        }
        "Timeline Ruler\nClick or drag to scrub playhead. Scroll to zoom time." => {
            "خط‌کش خط زمانی\nبرای پیمایش کلیک یا درگ کنید؛ برای بزرگ‌نمایی چرخ ماوس را بچرخانید."
        }
        _ => english,
    };
    visual_text(language, translated)
}

/// Egui 0.35+ performs native contextual text shaping and RTL layout via
/// HarfRust. Application copy retains logical Unicode without manual
/// presentation-form reordering.
pub fn visual_text(_language: AppLanguage, logical: &str) -> String {
    logical.to_owned()
}

/// Whether `text` contains Arabic-script Unicode, including Persian-specific
/// letters and the presentation forms used by the static-label compatibility
/// renderer.
pub fn contains_arabic_script(text: &str) -> bool {
    text.chars().any(|character| {
        matches!(
            character,
            '\u{0600}'..='\u{06FF}'
                | '\u{0750}'..='\u{077F}'
                | '\u{08A0}'..='\u{08FF}'
                | '\u{FB50}'..='\u{FDFF}'
                | '\u{FE70}'..='\u{FEFF}'
        )
    })
}

/// Keep editable Persian/Arabic content on its natural edge. Since egui 0.35,
/// HarfRust performs contextual shaping for `TextEdit`; this alignment helper
/// supplies the missing field-level RTL placement without changing the stored
/// logical Unicode or corrupting cursor/edit operations.
pub fn input_alignment(rtl_ui: bool, logical: &str) -> egui::Align {
    if rtl_ui || contains_arabic_script(logical) {
        egui::Align::Max
    } else {
        egui::Align::Min
    }
}

pub fn layout(rtl: bool, main_align: egui::Align) -> egui::Layout {
    if rtl {
        egui::Layout::right_to_left(main_align)
    } else {
        egui::Layout::left_to_right(main_align)
    }
}

pub fn vertical_layout(rtl: bool) -> egui::Layout {
    egui::Layout::top_down(if rtl {
        egui::Align::Max
    } else {
        egui::Align::Min
    })
}

/// Application copy only; protocol IDs, hostnames and peer names stay logical.
pub fn text(english: &str) -> String {
    tr(
        crate::config::resolved_language(&crate::config::live()),
        english,
    )
}

pub fn rtl() -> bool {
    crate::config::resolved_rtl(&crate::config::live())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn logical_unicode_and_application_copy() {
        assert_eq!(tr(AppLanguage::Persian, "Connect"), "اتصال");
        assert_eq!(tr(AppLanguage::English, "Connect"), "Connect");
        assert_eq!(tr(AppLanguage::Persian, "رلهٔ اصلی ۱۲۳"), "رلهٔ اصلی ۱۲۳");
        assert_eq!(visual_text(AppLanguage::English, "سلام دنیا"), "سلام دنیا");
        assert_eq!(input_alignment(false, "رلهٔ اصلی"), egui::Align::Max);
        assert_eq!(input_alignment(false, "ledboard.local"), egui::Align::Min);
    }
    #[test]
    fn changing_language_preserves_widget_fonts_and_monospace_fallback() {
        let context = egui::Context::default();
        configure_ui_fonts(&context, false);
        context
            .run_ui(Default::default(), |_| {})
            .drop_without_applying_deltas();
        elegance::Theme::current(&context).install(&context);
        context
            .run_ui(Default::default(), |_| {})
            .drop_without_applying_deltas();
        configure_ui_fonts(&context, true);
        context
            .run_ui(Default::default(), |ui| {
                ui.fonts(|fonts| {
                    let defs = fonts.definitions();
                    assert!(defs.font_data.contains_key("elegance-symbols"));
                    assert_eq!(
                        defs.families[&egui::FontFamily::Proportional][0],
                        "pealayer-vazirmatn"
                    );
                    assert_eq!(
                        defs.families[&egui::FontFamily::Monospace]
                            .iter()
                            .filter(|name| *name == "pealayer-vazirmatn")
                            .count(),
                        1
                    );
                });
            })
            .drop_without_applying_deltas();
        configure_ui_fonts(&context, false);
        context
            .run_ui(Default::default(), |ui| {
                ui.fonts(|fonts| {
                    assert!(
                        fonts
                            .definitions()
                            .font_data
                            .contains_key("elegance-symbols")
                    )
                });
            })
            .drop_without_applying_deltas();
    }
    // Actual Pealayer contextual-shaping test, expanded to the missing family.
    #[test]
    fn contextual_persian_in_labels_and_monospace_inputs() {
        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            let context = egui::Context::default();
            configure_ui_fonts(&context, true);
            let mut joined = Vec::new();
            let mut isolated = None;
            let mut output = context.run_ui(Default::default(), |ui| {
                let font = egui::FontId::new(16.0, family.clone());
                ui.fonts_mut(|fonts| {
                    for glyph in "سلام دنیا پ چ ژ گ ی ک ۱۲۳"
                        .chars()
                        .filter(|c| !c.is_whitespace())
                    {
                        assert!(
                            fonts.has_glyph(&font, glyph),
                            "Missing Persian glyph {glyph:?} in {family:?}"
                        );
                    }
                });
                joined = ui
                    .painter()
                    .layout_no_wrap("بب".into(), font.clone(), egui::Color32::WHITE)
                    .rows[0]
                    .glyphs
                    .iter()
                    .map(|glyph| glyph.uv_rect)
                    .collect();
                isolated = Some(
                    ui.painter()
                        .layout_no_wrap("ب".into(), font, egui::Color32::WHITE)
                        .rows[0]
                        .glyphs[0]
                        .uv_rect,
                );
            });
            output.textures_delta.clear();
            assert!(joined.len() >= 2);
            assert!(
                joined.iter().any(|glyph| Some(*glyph) != isolated),
                "Persian must join contextually in {family:?}"
            );
        }
    }
}
