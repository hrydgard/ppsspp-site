import re, math
W = '/private/tmp/claude-501/-Users-hrydg-ppsspp/a74ee475-6243-47b0-8d84-8dd7fb5a3e82/scratchpad/'
SURF, INK, INK2, MUTED, GRID = '#fcfcfb', '#0b0b0b', '#52514e', '#8a8984', '#e6e5e0'
S1, S2, S3, S4 = '#2a78d6', '#eb6834', '#1baf7a', '#eda100'
FONT = 'font-family="-apple-system, Segoe UI, Helvetica, Arial, sans-serif"'

def svg(w, h, body, title):
    return ('<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d" %s role="img" aria-label="%s">\n'
            '<rect width="100%%" height="100%%" fill="%s"/>\n%s</svg>\n') % (w, h, w, h, FONT, title, SURF, body)

def text(x, y, s, size=12, color=INK2, anchor='start', weight='normal'):
    return '<text x="%.1f" y="%.1f" font-size="%d" fill="%s" text-anchor="%s" font-weight="%s">%s</text>\n' % (x, y, size, color, anchor, weight, s)

def axes(x0, y0, pw, ph, xmin, xmax, ymin, ymax, xticks, yticks, xfmt=str, yfmt=str):
    X = lambda v: x0 + (v - xmin) / (xmax - xmin) * pw
    Y = lambda v: y0 + ph - (v - ymin) / (ymax - ymin) * ph
    out = ''
    for t in yticks:
        out += '<line x1="%.1f" x2="%.1f" y1="%.1f" y2="%.1f" stroke="%s" stroke-width="1"/>\n' % (x0, x0 + pw, Y(t), Y(t), GRID)
        out += text(x0 - 6, Y(t) + 4, yfmt(t), 11, MUTED, 'end')
    for t in xticks:
        out += text(X(t), y0 + ph + 16, xfmt(t), 11, MUTED, 'middle')
    return out, X, Y

def polyline(pts, color, width=2):
    return '<polyline fill="none" stroke="%s" stroke-width="%d" stroke-linejoin="round" stroke-linecap="round" points="%s"/>\n' % (color, width, ' '.join('%.1f,%.1f' % p for p in pts))

# Figure 1: the sawtooth, small multiples.
def fig1():
    fns = [('rcp', '1/x'), ('exp2', '2^x'), ('sqrt', '√x'), ('rsqrt', '1/√x')]
    w, h = 760, 430; pw, ph = 300, 120
    body = text(20, 28, 'The fingerprint: each 64-input interval is off by a floored squared term', 15, INK, weight='600')
    body += text(20, 48, 'Per-interval value minus a smooth quadratic, segment 64, first 160 intervals (units: 24-bit ulps)', 12, INK2)
    for idx, (fn, label) in enumerate(fns):
        data = [tuple(map(float, l.split(','))) for l in open(W + 'writeup/saw_%s.csv' % fn)][:160]
        x0 = 60 + (idx % 2) * 360; y0 = 90 + (idx // 2) * 170
        a, X, Y = axes(x0, y0, pw, ph, 0, 160, -0.6, 0.6, [0, 40, 80, 120, 160], [-0.5, 0, 0.5], yfmt=lambda v: '%+.1f' % v if v else '0')
        body += a + text(x0, y0 - 8, '%s (%s)' % (fn, label), 13, INK, weight='600')
        body += polyline([(X(k), Y(v)) for k, v in data], S1, 2)
    body += text(20, h - 14, 'Interval index within the segment. Each steady ramp wraps by one ulp: the floor of a slowly changing squared term.', 11, MUTED)
    return svg(w, h, body, 'Sawtooth of per-interval errors for rcp, exp2, sqrt and rsqrt')

# Figure 2: the squared-term coefficient n per segment.
def fig2():
    coef = {}
    cur = None
    for l in open(W + 'oracle/coefs.h'):
        m = re.match(r'static const VfpuSeg seg_(\w+)\[128\]', l)
        if m: cur = m.group(1); coef[cur] = []; continue
        m = re.match(r'\s*\{(-?\d+)LL, (-?\d+), (-?\d+), (\d+)\},', l)
        if m: coef[cur].append(int(m.group(3)))
    series = [('rsqrt', S1), ('rcp', S2), ('exp2', S3), ('sqrt', S4)]
    w, h = 760, 450; x0, y0, pw, ph = 70, 70, 620, 290
    body = text(20, 28, 'n, the squared-term coefficient, for each of the 128 segments', 15, INK, weight='600')
    body += text(20, 48, 'A 7-8 bit integer that tracks each function\'s curvature. rsqrt and sqrt restart at 64, where their index moves to the next binade', 12, INK2)
    a, X, Y = axes(x0, y0, pw, ph, 0, 127, -80, 200, [0, 32, 64, 96, 127], [-50, 0, 50, 100, 150, 200])
    body += a
    lx = 20
    for name, color in series:
        pts = [(X(g), Y(n)) for g, n in enumerate(coef[name])]
        body += polyline(pts, color)
        if name == 'rsqrt':
            lx, ly = pts[6]; body += text(lx + 12, ly + 2, name, 12, INK2)
        else:
            lx, ly = pts[22]; body += text(lx, ly - 8, name, 12, INK2, 'middle')
    # legend
    for i, (name, color) in enumerate(series):
        lx0 = x0 + i * 110
        body += '<line x1="%d" x2="%d" y1="%d" y2="%d" stroke="%s" stroke-width="2" stroke-linecap="round"/>\n' % (lx0, lx0 + 18, h - 20, h - 20, color)
        body += text(lx0 + 24, h - 16, name, 12, INK2)
    body += text(x0 + pw / 2, y0 + ph + 34, 'segment (top 7 bits of the input)', 11, MUTED, 'middle')
    return svg(w, h, body, 'Squared-term coefficient per segment for rsqrt, rcp, exp2 and sqrt')

# Figure 3: what the squarer computes.
def fig3():
    rows = [tuple(map(float, l.split(','))) for l in open(W + 'writeup/squarer.csv')]
    rows = [r for r in rows if 24 <= r[0] <= 96]
    w, h = 760, 420; x0, y0, pw, ph = 70, 70, 620, 270
    body = text(20, 28, 'Reading the squarer\'s output off 109 different coefficients', 15, INK, weight='600')
    body += text(20, 48, 'Bars: every value of T(t) - t² consistent with all of them. Dots: ceil(t²/256)·256 - t²', 12, INK2)
    a, X, Y = axes(x0, y0, pw, ph, 23, 97, 0, 280, [24, 40, 56, 72, 88], [0, 64, 128, 192, 256])
    body += a
    for t, l, hh in rows:
        body += '<rect x="%.1f" y="%.1f" width="5" height="%.1f" rx="2" fill="%s" fill-opacity="0.35"/>\n' % (X(t) - 2.5, Y(min(hh, 280)), max(2, Y(max(l, 0)) - Y(min(hh, 280))), S1)
    for t, l, hh in rows:
        v = math.ceil(t * t / 256) * 256 - t * t
        body += '<circle cx="%.1f" cy="%.1f" r="3.5" fill="%s" stroke="%s" stroke-width="2"/>\n' % (X(t), Y(v), S2, SURF)
    body += text(x0 + pw / 2, y0 + ph + 34, 't, distance from the middle of the segment, in intervals of 64 inputs', 11, MUTED, 'middle')
    body += '<rect x="%d" y="%d" width="10" height="14" rx="2" fill="%s" fill-opacity="0.35"/>\n' % (x0, h - 28, S1)
    body += text(x0 + 16, h - 17, 'range allowed by the data', 12, INK2)
    body += '<circle cx="%d" cy="%d" r="4" fill="%s"/>\n' % (x0 + 210, h - 21, S2)
    body += text(x0 + 220, h - 17, 'ceil(t²/256)·256 - t²', 12, INK2)
    return svg(w, h, body, 'The squarer output recovered from data, against ceil(t squared over 256) times 256')

# Figure 4: log2's output step by input exponent.
def fig4():
    w, h = 760, 360; x0, y0, pw, ph = 70, 70, 620, 220
    body = text(20, 28, 'log2: the output step depends on the input\'s exponent', 15, INK, weight='600')
    body += text(20, 48, 'log2 of the step size. Coefficients lose low bits to match the step; negative exponents use one fixed step', 12, INK2)
    a, X, Y = axes(x0, y0, pw, ph, -16, 128, -23, -14, [-16, 0, 32, 64, 96, 128], [-22, -20, -18, -16, -14])
    body += a
    pts = []
    for e in range(-16, 128):
        if e < 0: s = -15
        elif e < 2: s = -22
        else: s = -22 + int(math.log2(e))
        pts += [(X(e), Y(s)), (X(e + 1), Y(s))]
    body += polyline(pts, S1, 2)
    body += text(X(-8), Y(-15) - 8, 'negative exponents: 2^-15', 11, INK2, 'middle')
    body += text(X(1), Y(-22) + 16, '0 and 1: 2^-22', 11, INK2, 'start')
    body += text(x0 + pw / 2, y0 + ph + 34, 'input exponent', 11, MUTED, 'middle')
    return svg(w, h, body, 'log2 output step by input exponent')

for name, f in (('fig1-sawtooth', fig1), ('fig2-coefficients', fig2), ('fig3-squarer', fig3), ('fig4-log2-step', fig4)):
    open(W + 'writeup/%s.svg' % name, 'w').write(f())
print('ok')
