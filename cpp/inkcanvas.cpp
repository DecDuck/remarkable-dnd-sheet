#include "inkcanvas.h"
#include "epscreenmodeitem.h"

#include <QPainter>

#include <algorithm>
#include <cmath>

InkCanvas::InkCanvas(QQuickItem *parent)
    : QQuickPaintedItem(parent)
{
    setRenderTarget(QQuickPaintedItem::Image);
    setOpaquePainting(false);
    setFillColor(Qt::transparent);
    setAntialiasing(false);
    setMipmap(false);
}

void InkCanvas::setPenWidth(qreal w)
{
    if (qFuzzyCompare(m_penWidth, w)) {
        return;
    }
    m_penWidth = w;
    emit penWidthChanged();
}

void InkCanvas::setColor(const QColor &c)
{
    if (m_color == c) {
        return;
    }
    m_color = c;
    emit colorChanged();
}

void InkCanvas::componentComplete()
{
    QQuickPaintedItem::componentComplete();

    // Cover this item with an EPScreenModeItem in "Pen" mode so the epaper
    // compositor uses the fast handwriting waveform for this region.
    m_fastMode = new EPScreenModeItem(this);
    m_fastMode->setMode(EPScreenModeItem::Pen);
    syncFastMode();
}

void InkCanvas::syncFastMode()
{
    if (m_fastMode) {
        m_fastMode->setWidth(width());
        m_fastMode->setHeight(height());
    }
}

void InkCanvas::geometryChange(const QRectF &newGeometry, const QRectF &oldGeometry)
{
    QQuickPaintedItem::geometryChange(newGeometry, oldGeometry);
    setContentsSize(newGeometry.size().toSize());
    syncFastMode();
}

QRectF InkCanvas::segmentBounds(const Segment &s) const
{
    const qreal m = m_penWidth / 2.0 + 1.0;
    const qreal minX = std::min(s.a.x(), s.b.x()) - m;
    const qreal minY = std::min(s.a.y(), s.b.y()) - m;
    const qreal maxX = std::max(s.a.x(), s.b.x()) + m;
    const qreal maxY = std::max(s.a.y(), s.b.y()) + m;
    return QRectF(minX, minY, maxX - minX, maxY - minY);
}

void InkCanvas::moveTo(qreal x, qreal y)
{
    m_last = QPointF(x, y);
    m_hasLast = true;
}

void InkCanvas::lineTo(qreal x, qreal y)
{
    const QPointF p(x, y);
    if (!m_hasLast) {
        m_last = p;
        m_hasLast = true;
        return;
    }

    const Segment seg{m_last, p};
    m_segments.append(seg);
    m_last = p;

    update(segmentBounds(seg).toAlignedRect());
}

void InkCanvas::clear()
{
    m_segments.clear();
    m_hasLast = false;
    update();
}

void InkCanvas::eraseAt(qreal x, qreal y, qreal radius)
{
    // Remove every segment that passes within `radius` pixels of (x, y).
    // A segment AB is "hit" when the distance from (x,y) to the closest point
    // on the line AB is less than radius.  We use the standard point-to-segment
    // distance formula.
    const QPointF p(x, y);
    const qreal r2 = radius * radius;

    QRectF dirty;
    bool any = false;

    auto pointToSegmentDist2 = [](QPointF p, QPointF a, QPointF b) -> qreal {
        const QPointF ab = b - a;
        const qreal len2 = ab.x() * ab.x() + ab.y() * ab.y();
        if (len2 < 1e-10) {
            // Degenerate segment — just distance to a
            QPointF d = p - a;
            return d.x() * d.x() + d.y() * d.y();
        }
        const qreal t = qBound(0.0,
            ((p.x() - a.x()) * ab.x() + (p.y() - a.y()) * ab.y()) / len2,
            1.0);
        const QPointF proj(a.x() + t * ab.x(), a.y() + t * ab.y());
        const QPointF d = p - proj;
        return d.x() * d.x() + d.y() * d.y();
    };

    QList<Segment> kept;
    kept.reserve(m_segments.size());

    for (const Segment &seg : m_segments) {
        if (pointToSegmentDist2(p, seg.a, seg.b) < r2) {
            const QRectF sb = segmentBounds(seg);
            dirty = any ? dirty.united(sb) : sb;
            any = true;
        } else {
            kept.append(seg);
        }
    }

    if (any) {
        m_segments = std::move(kept);
        // Also invalidate the eraser circle itself so the white erase region
        // is repainted even when no segments were near the boundary.
        const QRectF eraserRect(x - radius, y - radius, radius * 2, radius * 2);
        dirty = dirty.united(eraserRect);
        update(dirty.adjusted(-1, -1, 1, 1).toAlignedRect());
    }

    // Always reset the draw cursor; the eraser tip doesn't start a new stroke.
    m_hasLast = false;
}

void InkCanvas::paint(QPainter *painter)
{
    if (m_segments.isEmpty()) {
        return;
    }

    QRectF clip = painter->clipBoundingRect();
    const bool haveClip = clip.isValid() && !clip.isEmpty();

    painter->setRenderHint(QPainter::Antialiasing, false);
    QPen pen(m_color);
    pen.setWidthF(m_penWidth);
    pen.setCapStyle(Qt::RoundCap);
    pen.setJoinStyle(Qt::RoundJoin);
    painter->setPen(pen);

    for (const Segment &seg : m_segments) {
        if (haveClip && !segmentBounds(seg).intersects(clip)) {
            continue;
        }
        painter->drawLine(seg.a, seg.b);
    }
}
