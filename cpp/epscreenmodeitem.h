#pragma once

#include <QQuickItem>

// Minimal, ABI-compatible declaration of the private QQuickItem type that the
// reMarkable epaper scenegraph plugin (libqsgepaper.so) exports. Covering a
// region with one of these tells the epaper compositor which e-ink waveform to
// use; "Pen" is the fast handwriting waveform the native UI uses.
//
// No Q_OBJECT: the metaobject, constructor, setMode() and vtable all come from
// libqsgepaper.so. We link against it so these symbols resolve.
class EPScreenModeItem : public QQuickItem
{
public:
    enum Mode {
        Pen = 0,
        Mono = 1,
        Animation = 2,
        UI = 3,
        Content = 4,
        Sleep = 5
    };

    explicit EPScreenModeItem(QQuickItem *parent = nullptr);

    Mode mode() const;
    void setMode(Mode mode);
};
