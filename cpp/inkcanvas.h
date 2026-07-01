#pragma once

#include <QColor>
#include <QList>
#include <QPointF>
#include <QRectF>
#include <QQuickPaintedItem>

class EPScreenModeItem;

// Ink surface built on QQuickPaintedItem, the draw path the epaper scene-graph
// backend is designed for (EPContext::createPainterNode) and the one the native
// UI uses. Each stroke segment calls update(smallRect), so the backing store
// and the e-ink panel only refresh a tiny region per stroke, matching native
// handwriting speed.
//
// On completion the item covers itself with an EPScreenModeItem in "Pen" mode
// (from libqsgepaper.so) to select the fast handwriting waveform.
class InkCanvas : public QQuickPaintedItem
{
    Q_OBJECT
    Q_PROPERTY(qreal penWidth READ penWidth WRITE setPenWidth NOTIFY penWidthChanged)
    Q_PROPERTY(QColor color READ color WRITE setColor NOTIFY colorChanged)

public:
    explicit InkCanvas(QQuickItem *parent = nullptr);

    qreal penWidth() const { return m_penWidth; }
    void setPenWidth(qreal w);

    QColor color() const { return m_color; }
    void setColor(const QColor &c);

    Q_INVOKABLE void moveTo(qreal x, qreal y);
    Q_INVOKABLE void lineTo(qreal x, qreal y);
    Q_INVOKABLE void eraseAt(qreal x, qreal y, qreal radius);
    Q_INVOKABLE void clear();

    void paint(QPainter *painter) override;

signals:
    void penWidthChanged();
    void colorChanged();

protected:
    void componentComplete() override;
    void geometryChange(const QRectF &newGeometry, const QRectF &oldGeometry) override;

private:
    struct Segment {
        QPointF a;
        QPointF b;
    };

    QRectF segmentBounds(const Segment &s) const;
    void syncFastMode();

    qreal m_penWidth = 3.0;
    QColor m_color = Qt::black;

    QList<Segment> m_segments;
    QPointF m_last;
    bool m_hasLast = false;

    EPScreenModeItem *m_fastMode = nullptr;
};
