import numpy as np
from itertools import product, combinations
import matplotlib.pyplot as plt

data = [
    (-0.0292149526928, -0.0241923869131, 0.52269561933),
    (-0.0288222339759, -0.0306361018019, 0.525505113107),
    (-0.0283090812583, -0.0366442193256, 0.529139415773),
    (-0.0276846518951, -0.0421095229316, 0.533533672172),
    (-0.0269600886818, -0.0469344855587, 0.53860946783),
    (-0.0213278189953, -0.0585886486063, 0.577671141223),
    (-0.0203322512475, -0.0573229419388, 0.584525028903),
    (-0.0193677533749, -0.0551454095765, 0.591150514125),
    (-0.0184515371141, -0.052094910199, 0.597429363235),
    (-0.0175999521295, -0.0482258792521, 0.603249531644),
    (-0.0168281951954, -0.0436073606852, 0.608507156804),
    (-0.0161500382152, -0.0383217716083, 0.613108417072),
    (-0.0193474918165, 0.04321050765, 0.589790751867),
    (-0.020311130356, 0.0452044463785, 0.583107596409),
    (-0.0213062156734, 0.0462806066905, 0.576221332616),
    (-0.022314990573, 0.046419784801, 0.569254845159),
    (-0.023319453575, 0.0456194969081, 0.56233245026),
    (-0.0243016797537, 0.0438940243752, 0.555577680012),
    (-0.0252441415132, 0.0412741578549, 0.549111071704),
    (-0.0261300203575, 0.0378066496047, 0.543048023786),
    (-0.0269435079864, 0.033553376496, 0.537496729928),
    (-0.0276700874835, 0.0285902393879, 0.532556254534),
    (-0.0282967932082, 0.0230058042978, 0.528314759229),
    (-0.02881244142, 0.0168997270462, 0.524847934694),
    (-0.0292078304973, 0.0103809691492, 0.522217645599),
    (-0.0294759046107, 0.00356585968151, 0.520470830197),
    (-0.0296118800403, -0.00342398732706, 0.519638659997),
    (-0.0296133302663, -0.0104638360839, 0.519735985312),
    (-0.0294802294097, -0.017428062139, 0.520761069394),
    (-0.0292149526928, -0.0241923869131, 0.52269561933),
    (-0.0289726466043, -0.0284423587627, 0.524433248357),
    (0.0253214159027, -0.0293792487736, 0.618360066),
    (0.0251047679644, -0.0251578300064, 0.620169213551),
    (0.0248682934353, -0.0184255570846, 0.6222160467),
    (0.02475084721, -0.0114787519258, 0.6233549331),
    (0.0247545251495, -0.00444138222871, 0.623565549002),
    (0.0248792616217, 0.00256097179509, 0.622844136013),
    (0.0251228306692, 0.00940335116174, 0.621203567953),
    (0.0254808858594, 0.0159636552164, 0.618673120373),
    (0.0313386715547, 0.0461955496235, 0.574657858455),
    (0.0322465423181, 0.0461686603603, 0.567676168654),
    (0.028650693588, -0.0548464785114, 0.59189613682),
    (0.0278557511485, -0.0516471860246, 0.598117193954),
    (0.0271265258805, -0.0476405218956, 0.603860347957),
    (0.0264760306379, -0.042897984106, 0.609023113594),
    (0.0259158736918, -0.0375042045812, 0.613513359853),
    (0.0254560509115, -0.0315554340517, 0.617250959345),
]

ax = plt.axes(projection="3d")
# Data for three-dimensional scattered points
l = [i for i in range(len(data))]
xdata = [x for (x, y, z) in data]
ydata = [y for (x, y, z) in data]
zdata = [z for (x, y, z) in data]

ax.scatter3D(xdata, ydata, zdata, c=l, cmap="Greens")

x = [-0.023121, 0.078626]
y = [-0.038009, 0.121636]
z = [-0.091940, -0.017395]
v = [
    (x[0], y[0], z[0]),
    (x[0], y[0], z[1]),
    (x[0], y[1], z[0]),
    (x[0], y[1], z[1]),
    (x[1], y[0], z[0]),
    (x[1], y[0], z[1]),
    (x[1], y[1], z[0]),
    (x[1], y[1], z[1]),
]

# draw cube
r = [-1, 1]
for s, e in combinations(np.array(v), 2):
    if True or np.sum(np.abs(s - e)) == r[1] - r[0]:
        ax.plot3D(*zip(s, e), color="b")

# (-0.023121 -0.038009 -0.091940)
# (0.078626 0.121636 -0.017395)


# a = []

# ax.scatter3D(a, b, c, c=zdata, cmap="Reds")

plt.show()