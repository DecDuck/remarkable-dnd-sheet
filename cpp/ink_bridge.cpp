#include "inkcanvas.h"
#include "epscreenmodeitem.h"

#include <QObject>
#include <QQuickItem>
#include <QQuickWindow>
#include <QTimer>
#include <qqml.h>

// Triggers a one-shot full refresh when the active tab changes to clear
// e-ink ghosting. No white flash: the tab-switch pixel changes themselves
// drive the waveform. Usage in QML:
//   ScreenRefresher { id: screenRefresher }
//   onCurrentTabChanged: screenRefresher.refresh(root.contentItem)
class ScreenRefresher : public QObject
{
    Q_OBJECT
public:
    explicit ScreenRefresher(QObject *parent = nullptr) : QObject(parent) {}

    // Cover the content area with an EPScreenModeItem in UI mode for a
    // single rendered frame. The new tab content provides the pixel changes;
    // the mode item just tells the epaper compositor to use the UI (GL16/fast)
    // waveform for that region instead of the pen waveform, which clears
    // ghosts without any extra visible flash.
    Q_INVOKABLE void refresh(QQuickItem *item) {
        if (!item || !item->window()) return;

        auto *mode = new EPScreenModeItem(item);
        mode->setX(0);
        mode->setY(0);
        mode->setWidth(item->width());
        mode->setHeight(item->height());
        mode->setMode(EPScreenModeItem::UI);

        // Schedule the frame that will pick up both the new tab content and
        // the waveform override simultaneously.
        item->window()->update();

        // One frame is enough for the waveform to be selected. Remove the
        // item before the next frame so pen drawing reverts to Pen mode.
        QTimer::singleShot(32, mode, [mode]() { delete mode; });
    }
};

// Pull in the moc output generated from this .cpp file (Q_OBJECT in-source).
#include "ink_bridge.moc"

// Registers the C++ ink rendering types so QML can instantiate them via
// `import InkTools 1.0`. Called from Rust main() before the QML is loaded.
extern "C" void register_ink_types()
{
    qmlRegisterType<InkCanvas>("InkTools", 1, 0, "InkCanvas");
    qmlRegisterType<ScreenRefresher>("InkTools", 1, 0, "ScreenRefresher");
}

// Q_INIT_RESOURCE must be called from a plain C++ function (NOT from within an
// extern "C" block). GCC propagates C linkage to inner `extern` declarations,
// so placing Q_INIT_RESOURCE inside an extern "C" function causes it to look
// for the unmangled symbol `qInitResources_charsheet`, while rcc exports the
// C++-mangled `_Z24qInitResources_charsheetv`. The solution is a C++ impl
// function + a thin extern "C" entry point that Rust can call.
static void init_charsheet_resources_impl()
{
    Q_INIT_RESOURCE(charsheet);
}

extern "C" void init_charsheet_resources()
{
    init_charsheet_resources_impl();
}
