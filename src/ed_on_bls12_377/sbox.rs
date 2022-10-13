use super::BigInteger256;
use super::Felt;

use ark_ff::Field;

#[allow(unused)]
/// Exponent of the Anemoi S-Box
pub(crate) const ALPHA: u32 = 11;

#[allow(unused)]
/// Inverse exponent
pub(crate) const INV_ALPHA: [u64; 4] = [
    0x655422e8ba2e8ba3,
    0xec45a72d92e8ba2f,
    0x37d9565ea88aa746,
    0x0f466a36210d4175,
];

/// Multiplier of the Anemoi S-Box
#[allow(unused)]
pub(crate) const BETA: u32 = 22;

/// First added constant of the Anemoi S-Box
pub(crate) const DELTA: Felt = Felt::new(BigInteger256([
    0xb76f9745d1745d17,
    0xfed18274afffffff,
    0xfce619835b36a173,
    0x068b6ffd78dc8d16,
]));

#[allow(unused)]
/// Second added constant of the Anemoi S-Box
pub(crate) const QUAD: u32 = 2;

#[inline(always)]
pub(crate) fn exp_inv_alpha(x: &Felt) -> Felt {
    let t10 = x.square(); //      1: 2
    let t0 = t10 * x; //          2: 3
    let t7 = t10.square(); //     3: 4
    let t14 = t0 * t10; //        4: 5
    let t0 = t7.square(); //      5: 8
    let t8 = t14 * t7; //         6: 9
    let t4 = t8 * t0; //          7: 17
    let t5 = t4 * t7; //          8: 21
    let t2 = t5 * t10; //         9: 23
    let t15 = t4 * t0; //        10: 25
    let t9 = t2 * t7; //         11: 27
    let t3 = t5 * t0; //         12: 29
    let t18 = t15 * t0; //       13: 33
    let t1 = t9 * t0; //         14: 35
    let t6 = t3 * t0; //         15: 37
    let t16 = t18 * t0; //       16: 41
    let t11 = t6 * t0; //        17: 45
    let t12 = t16 * t0; //       18: 49
    let t19 = t12 * t10; //      19: 51
    let t17 = t11 * t0; //       20: 53
    let t13 = t19 * t7; //       21: 55
    let t10 = t12 * t0; //       22: 57
    let t7 = t17 * t0; //        23: 61
    let mut t0 = t7.square(); // 24: 122
    t0 = t0.square(); //         25: 244
    t0 = t0.square(); //         26: 488
    t0 = t0.square(); //         27: 976
    t0 = t0.square(); //         28: 1952
    t0 = t0.square(); //         29: 3904
    t0 = t0.square(); //         30: 7808
    t0 = t0.square(); //         31: 15616
    t0 = t0.square(); //         32: 31232
    t0 *= t19; //                33: 31283
    t0 = t0.square(); //         34: 62566
    t0 = t0.square(); //         35: 125132
    t0 = t0.square(); //         36: 250264
    t0 = t0.square(); //         37: 500528
    t0 *= t14; //                38: 500533
    t0 = t0.square(); //         39: 1001066
    t0 = t0.square(); //         40: 2002132
    t0 = t0.square(); //         41: 4004264
    t0 = t0.square(); //         42: 8008528
    t0 = t0.square(); //         43: 16017056
    t0 = t0.square(); //         44: 32034112
    t0 = t0.square(); //         45: 64068224
    t0 = t0.square(); //         46: 128136448
    t0 *= t9; //                 47: 128136475
    t0 = t0.square(); //         48: 256272950
    t0 = t0.square(); //         49: 512545900
    t0 = t0.square(); //         50: 1025091800
    t0 = t0.square(); //         51: 2050183600
    t0 = t0.square(); //         52: 4100367200
    t0 = t0.square(); //         53: 8200734400
    t0 = t0.square(); //         54: 16401468800
    t0 = t0.square(); //         55: 32802937600
    t0 = t0.square(); //         56: 65605875200
    t0 *= t18; //                57: 65605875233
    t0 = t0.square(); //         58: 131211750466
    t0 = t0.square(); //         59: 262423500932
    t0 = t0.square(); //         60: 524847001864
    t0 = t0.square(); //         61: 1049694003728
    t0 = t0.square(); //         62: 2099388007456
    t0 = t0.square(); //         63: 4198776014912
    t0 = t0.square(); //         64: 8397552029824
    t0 = t0.square(); //         65: 16795104059648
    t0 = t0.square(); //         66: 33590208119296
    t0 = t0.square(); //         67: 67180416238592
    t0 *= t17; //                68: 67180416238645
    t0 = t0.square(); //         69: 134360832477290
    t0 = t0.square(); //         70: 268721664954580
    t0 = t0.square(); //         71: 537443329909160
    t0 = t0.square(); //         72: 1074886659818320
    t0 = t0.square(); //         73: 2149773319636640
    t0 = t0.square(); //         74: 4299546639273280
    t0 = t0.square(); //         75: 8599093278546560
    t0 = t0.square(); //         76: 17198186557093120
    t0 = t0.square(); //         77: 34396373114186240
    t0 = t0.square(); //         78: 68792746228372480
    t0 *= t2; //                 79: 68792746228372503
    t0 = t0.square(); //         80: 137585492456745006
    t0 = t0.square(); //         81: 275170984913490012
    t0 = t0.square(); //         82: 550341969826980024
    t0 = t0.square(); //         83: 1100683939653960048
    t0 = t0.square(); //         84: 2201367879307920096
    t0 = t0.square(); //         85: 4402735758615840192
    t0 = t0.square(); //         86: 8805471517231680384
    t0 *= t16; //                87: 8805471517231680425
    t0 = t0.square(); //         88: 17610943034463360850
    t0 = t0.square(); //         89: 35221886068926721700
    t0 = t0.square(); //         90: 70443772137853443400
    t0 = t0.square(); //         91: 140887544275706886800
    t0 = t0.square(); //         92: 281775088551413773600
    t0 *= t2; //                 93: 281775088551413773623
    t0 = t0.square(); //         94: 563550177102827547246
    t0 = t0.square(); //         95: 1127100354205655094492
    t0 = t0.square(); //         96: 2254200708411310188984
    t0 = t0.square(); //         97: 4508401416822620377968
    t0 = t0.square(); //         98: 9016802833645240755936
    t0 *= t9; //                 99: 9016802833645240755963
    t0 = t0.square(); //        100: 18033605667290481511926
    t0 = t0.square(); //        101: 36067211334580963023852
    t0 = t0.square(); //        102: 72134422669161926047704
    t0 = t0.square(); //        103: 144268845338323852095408
    t0 = t0.square(); //        104: 288537690676647704190816
    t0 = t0.square(); //        105: 577075381353295408381632
    t0 = t0.square(); //        106: 1154150762706590816763264
    t0 *= t5; //                107: 1154150762706590816763285
    t0 = t0.square(); //        108: 2308301525413181633526570
    t0 = t0.square(); //        109: 4616603050826363267053140
    t0 = t0.square(); //        110: 9233206101652726534106280
    t0 = t0.square(); //        111: 18466412203305453068212560
    t0 = t0.square(); //        112: 36932824406610906136425120
    t0 = t0.square(); //        113: 73865648813221812272850240
    t0 *= t15; //               114: 73865648813221812272850265
    t0 = t0.square(); //        115: 147731297626443624545700530
    t0 = t0.square(); //        116: 295462595252887249091401060
    t0 = t0.square(); //        117: 590925190505774498182802120
    t0 = t0.square(); //        118: 1181850381011548996365604240
    t0 = t0.square(); //        119: 2363700762023097992731208480
    t0 = t0.square(); //        120: 4727401524046195985462416960
    t0 = t0.square(); //        121: 9454803048092391970924833920
    t0 *= t7; //                122: 9454803048092391970924833981
    t0 = t0.square(); //        123: 18909606096184783941849667962
    t0 = t0.square(); //        124: 37819212192369567883699335924
    t0 = t0.square(); //        125: 75638424384739135767398671848
    t0 = t0.square(); //        126: 151276848769478271534797343696
    t0 *= t14; //               127: 151276848769478271534797343701
    t0 = t0.square(); //        128: 302553697538956543069594687402
    t0 = t0.square(); //        129: 605107395077913086139189374804
    t0 = t0.square(); //        130: 1210214790155826172278378749608
    t0 = t0.square(); //        131: 2420429580311652344556757499216
    t0 = t0.square(); //        132: 4840859160623304689113514998432
    t0 = t0.square(); //        133: 9681718321246609378227029996864
    t0 = t0.square(); //        134: 19363436642493218756454059993728
    t0 = t0.square(); //        135: 38726873284986437512908119987456
    t0 *= t4; //                136: 38726873284986437512908119987473
    t0 = t0.square(); //        137: 77453746569972875025816239974946
    t0 = t0.square(); //        138: 154907493139945750051632479949892
    t0 = t0.square(); //        139: 309814986279891500103264959899784
    t0 = t0.square(); //        140: 619629972559783000206529919799568
    t0 = t0.square(); //        141: 1239259945119566000413059839599136
    t0 = t0.square(); //        142: 2478519890239132000826119679198272
    t0 *= t5; //                143: 2478519890239132000826119679198293
    t0 = t0.square(); //        144: 4957039780478264001652239358396586
    t0 = t0.square(); //        145: 9914079560956528003304478716793172
    t0 = t0.square(); //        146: 19828159121913056006608957433586344
    t0 = t0.square(); //        147: 39656318243826112013217914867172688
    t0 = t0.square(); //        148: 79312636487652224026435829734345376
    t0 = t0.square(); //        149: 158625272975304448052871659468690752
    t0 = t0.square(); //        150: 317250545950608896105743318937381504
    t0 *= t3; //                151: 317250545950608896105743318937381533
    t0 = t0.square(); //        152: 634501091901217792211486637874763066
    t0 = t0.square(); //        153: 1269002183802435584422973275749526132
    t0 = t0.square(); //        154: 2538004367604871168845946551499052264
    t0 = t0.square(); //        155: 5076008735209742337691893102998104528
    t0 = t0.square(); //        156: 10152017470419484675383786205996209056
    t0 = t0.square(); //        157: 20304034940838969350767572411992418112
    t0 = t0.square(); //        158: 40608069881677938701535144823984836224
    t0 = t0.square(); //        159: 81216139763355877403070289647969672448
    t0 = t0.square(); //        160: 162432279526711754806140579295939344896
    t0 *= t13; //               161: 162432279526711754806140579295939344951
    t0 = t0.square(); //        162: 324864559053423509612281158591878689902
    t0 = t0.square(); //        163: 649729118106847019224562317183757379804
    t0 = t0.square(); //        164: 1299458236213694038449124634367514759608
    t0 = t0.square(); //        165: 2598916472427388076898249268735029519216
    t0 = t0.square(); //        166: 5197832944854776153796498537470059038432
    t0 = t0.square(); //        167: 10395665889709552307592997074940118076864
    t0 = t0.square(); //        168: 20791331779419104615185994149880236153728
    t0 *= t12; //               169: 20791331779419104615185994149880236153777
    t0 = t0.square(); //        170: 41582663558838209230371988299760472307554
    t0 = t0.square(); //        171: 83165327117676418460743976599520944615108
    t0 = t0.square(); //        172: 166330654235352836921487953199041889230216
    t0 = t0.square(); //        173: 332661308470705673842975906398083778460432
    t0 = t0.square(); //        174: 665322616941411347685951812796167556920864
    t0 = t0.square(); //        175: 1330645233882822695371903625592335113841728
    t0 = t0.square(); //        176: 2661290467765645390743807251184670227683456
    t0 = t0.square(); //        177: 5322580935531290781487614502369340455366912
    t0 = t0.square(); //        178: 10645161871062581562975229004738680910733824
    t0 *= t11; //               179: 10645161871062581562975229004738680910733869
    t0 = t0.square(); //        180: 21290323742125163125950458009477361821467738
    t0 = t0.square(); //        181: 42580647484250326251900916018954723642935476
    t0 = t0.square(); //        182: 85161294968500652503801832037909447285870952
    t0 = t0.square(); //        183: 170322589937001305007603664075818894571741904
    t0 = t0.square(); //        184: 340645179874002610015207328151637789143483808
    t0 = t0.square(); //        185: 681290359748005220030414656303275578286967616
    t0 = t0.square(); //        186: 1362580719496010440060829312606551156573935232
    t0 = t0.square(); //        187: 2725161438992020880121658625213102313147870464
    t0 *= t10; //               188: 2725161438992020880121658625213102313147870521
    t0 = t0.square(); //        189: 5450322877984041760243317250426204626295741042
    t0 = t0.square(); //        190: 10900645755968083520486634500852409252591482084
    t0 = t0.square(); //        191: 21801291511936167040973269001704818505182964168
    t0 = t0.square(); //        192: 43602583023872334081946538003409637010365928336
    t0 = t0.square(); //        193: 87205166047744668163893076006819274020731856672
    t0 = t0.square(); //        194: 174410332095489336327786152013638548041463713344
    t0 *= t9; //                195: 174410332095489336327786152013638548041463713371
    t0 = t0.square(); //        196: 348820664190978672655572304027277096082927426742
    t0 = t0.square(); //        197: 697641328381957345311144608054554192165854853484
    t0 = t0.square(); //        198: 1395282656763914690622289216109108384331709706968
    t0 = t0.square(); //        199: 2790565313527829381244578432218216768663419413936
    t0 = t0.square(); //        200: 5581130627055658762489156864436433537326838827872
    t0 = t0.square(); //        201: 11162261254111317524978313728872867074653677655744
    t0 *= t8; //                202: 11162261254111317524978313728872867074653677655753
    t0 = t0.square(); //        203: 22324522508222635049956627457745734149307355311506
    t0 = t0.square(); //        204: 44649045016445270099913254915491468298614710623012
    t0 = t0.square(); //        205: 89298090032890540199826509830982936597229421246024
    t0 = t0.square(); //        206: 178596180065781080399653019661965873194458842492048
    t0 = t0.square(); //        207: 357192360131562160799306039323931746388917684984096
    t0 = t0.square(); //        208: 714384720263124321598612078647863492777835369968192
    t0 *= t3; //                209: 714384720263124321598612078647863492777835369968221
    t0 = t0.square(); //        210: 1428769440526248643197224157295726985555670739936442
    t0 = t0.square(); //        211: 2857538881052497286394448314591453971111341479872884
    t0 = t0.square(); //        212: 5715077762104994572788896629182907942222682959745768
    t0 = t0.square(); //        213: 11430155524209989145577793258365815884445365919491536
    t0 = t0.square(); //        214: 22860311048419978291155586516731631768890731838983072
    t0 = t0.square(); //        215: 45720622096839956582311173033463263537781463677966144
    t0 = t0.square(); //        216: 91441244193679913164622346066926527075562927355932288
    t0 = t0.square(); //        217: 182882488387359826329244692133853054151125854711864576
    t0 *= t2; //                218: 182882488387359826329244692133853054151125854711864599
    t0 = t0.square(); //        219: 365764976774719652658489384267706108302251709423729198
    t0 = t0.square(); //        220: 731529953549439305316978768535412216604503418847458396
    t0 = t0.square(); //        221: 1463059907098878610633957537070824433209006837694916792
    t0 = t0.square(); //        222: 2926119814197757221267915074141648866418013675389833584
    t0 = t0.square(); //        223: 5852239628395514442535830148283297732836027350779667168
    t0 = t0.square(); //        224: 11704479256791028885071660296566595465672054701559334336
    t0 *= t4; //                225: 11704479256791028885071660296566595465672054701559334353
    t0 = t0.square(); //        226: 23408958513582057770143320593133190931344109403118668706
    t0 = t0.square(); //        227: 46817917027164115540286641186266381862688218806237337412
    t0 = t0.square(); //        228: 93635834054328231080573282372532763725376437612474674824
    t0 = t0.square(); //        229: 187271668108656462161146564745065527450752875224949349648
    t0 = t0.square(); //        230: 374543336217312924322293129490131054901505750449898699296
    t0 = t0.square(); //        231: 749086672434625848644586258980262109803011500899797398592
    t0 = t0.square(); //        232: 1498173344869251697289172517960524219606023001799594797184
    t0 *= t7; //                233: 1498173344869251697289172517960524219606023001799594797245
    t0 = t0.square(); //        234: 2996346689738503394578345035921048439212046003599189594490
    t0 = t0.square(); //        235: 5992693379477006789156690071842096878424092007198379188980
    t0 = t0.square(); //        236: 11985386758954013578313380143684193756848184014396758377960
    t0 = t0.square(); //        237: 23970773517908027156626760287368387513696368028793516755920
    t0 = t0.square(); //        238: 47941547035816054313253520574736775027392736057587033511840
    t0 = t0.square(); //        239: 95883094071632108626507041149473550054785472115174067023680
    t0 *= t6; //                240: 95883094071632108626507041149473550054785472115174067023717
    t0 = t0.square(); //        241: 191766188143264217253014082298947100109570944230348134047434
    t0 = t0.square(); //        242: 383532376286528434506028164597894200219141888460696268094868
    t0 = t0.square(); //        243: 767064752573056869012056329195788400438283776921392536189736
    t0 = t0.square(); //        244: 1534129505146113738024112658391576800876567553842785072379472
    t0 = t0.square(); //        245: 3068259010292227476048225316783153601753135107685570144758944
    t0 = t0.square(); //        246: 6136518020584454952096450633566307203506270215371140289517888
    t0 *= t5; //                247: 6136518020584454952096450633566307203506270215371140289517909
    t0 = t0.square(); //        248: 12273036041168909904192901267132614407012540430742280579035818
    t0 = t0.square(); //        249: 24546072082337819808385802534265228814025080861484561158071636
    t0 = t0.square(); //        250: 49092144164675639616771605068530457628050161722969122316143272
    t0 = t0.square(); //        251: 98184288329351279233543210137060915256100323445938244632286544
    t0 = t0.square(); //        252: 196368576658702558467086420274121830512200646891876489264573088
    t0 = t0.square(); //        253: 392737153317405116934172840548243661024401293783752978529146176
    t0 = t0.square(); //        254: 785474306634810233868345681096487322048802587567505957058292352
    t0 = t0.square(); //        255: 1570948613269620467736691362192974644097605175135011914116584704
    t0 = t0.square(); //        256: 3141897226539240935473382724385949288195210350270023828233169408
    t0 *= t4; //                257: 3141897226539240935473382724385949288195210350270023828233169425
    t0 = t0.square(); //        258: 6283794453078481870946765448771898576390420700540047656466338850
    t0 = t0.square(); //        259: 12567588906156963741893530897543797152780841401080095312932677700
    t0 = t0.square(); //        260: 25135177812313927483787061795087594305561682802160190625865355400
    t0 = t0.square(); //        261: 50270355624627854967574123590175188611123365604320381251730710800
    t0 = t0.square(); //        262: 100540711249255709935148247180350377222246731208640762503461421600
    t0 = t0.square(); //        263: 201081422498511419870296494360700754444493462417281525006922843200
    t0 *= t3; //                264: 201081422498511419870296494360700754444493462417281525006922843229
    t0 = t0.square(); //        265: 402162844997022839740592988721401508888986924834563050013845686458
    t0 = t0.square(); //        266: 804325689994045679481185977442803017777973849669126100027691372916
    t0 = t0.square(); //        267: 1608651379988091358962371954885606035555947699338252200055382745832
    t0 = t0.square(); //        268: 3217302759976182717924743909771212071111895398676504400110765491664
    t0 = t0.square(); //        269: 6434605519952365435849487819542424142223790797353008800221530983328
    t0 = t0.square(); //        270: 12869211039904730871698975639084848284447581594706017600443061966656
    t0 = t0.square(); //        271: 25738422079809461743397951278169696568895163189412035200886123933312
    t0 = t0.square(); //        272: 51476844159618923486795902556339393137790326378824070401772247866624
    t0 *= t2; //                273: 51476844159618923486795902556339393137790326378824070401772247866647
    t0 = t0.square(); //        274: 102953688319237846973591805112678786275580652757648140803544495733294
    t0 = t0.square(); //        275: 205907376638475693947183610225357572551161305515296281607088991466588
    t0 = t0.square(); //        276: 411814753276951387894367220450715145102322611030592563214177982933176
    t0 = t0.square(); //        277: 823629506553902775788734440901430290204645222061185126428355965866352
    t0 = t0.square(); //        278: 1647259013107805551577468881802860580409290444122370252856711931732704
    t0 = t0.square(); //        279: 3294518026215611103154937763605721160818580888244740505713423863465408
    t0 *= t4; //                280: 3294518026215611103154937763605721160818580888244740505713423863465425
    t0 = t0.square(); //        281: 6589036052431222206309875527211442321637161776489481011426847726930850
    t0 = t0.square(); //        282: 13178072104862444412619751054422884643274323552978962022853695453861700
    t0 = t0.square(); //        283: 26356144209724888825239502108845769286548647105957924045707390907723400
    t0 = t0.square(); //        284: 52712288419449777650479004217691538573097294211915848091414781815446800
    t0 = t0.square(); //        285: 105424576838899555300958008435383077146194588423831696182829563630893600
    t0 = t0.square(); //        286: 210849153677799110601916016870766154292389176847663392365659127261787200
    t0 *= t3; //                287: 210849153677799110601916016870766154292389176847663392365659127261787229
    t0 = t0.square(); //        288: 421698307355598221203832033741532308584778353695326784731318254523574458
    t0 = t0.square(); //        289: 843396614711196442407664067483064617169556707390653569462636509047148916
    t0 = t0.square(); //        290: 1686793229422392884815328134966129234339113414781307138925273018094297832
    t0 = t0.square(); //        291: 3373586458844785769630656269932258468678226829562614277850546036188595664
    t0 = t0.square(); //        292: 6747172917689571539261312539864516937356453659125228555701092072377191328
    t0 = t0.square(); //        293: 13494345835379143078522625079729033874712907318250457111402184144754382656
    t0 = t0.square(); //        294: 26988691670758286157045250159458067749425814636500914222804368289508765312
    t0 = t0.square(); //        295: 53977383341516572314090500318916135498851629273001828445608736579017530624
    t0 *= t2; //                296: 53977383341516572314090500318916135498851629273001828445608736579017530647
    t0 = t0.square(); //        297: 107954766683033144628181000637832270997703258546003656891217473158035061294
    t0 = t0.square(); //        298: 215909533366066289256362001275664541995406517092007313782434946316070122588
    t0 = t0.square(); //        299: 431819066732132578512724002551329083990813034184014627564869892632140245176
    t0 = t0.square(); //        300: 863638133464265157025448005102658167981626068368029255129739785264280490352
    t0 = t0.square(); //        301: 1727276266928530314050896010205316335963252136736058510259479570528560980704
    t0 = t0.square(); //        302: 3454552533857060628101792020410632671926504273472117020518959141057121961408
    t0 = t0.square(); //        303: 6909105067714121256203584040821265343853008546944234041037918282114243922816
    t0 * t1 //                  304: 6909105067714121256203584040821265343853008546944234041037918282114243922851
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::One;

    #[test]
    fn test_alpha() {
        let mut a = -Felt::one();
        for _ in 0..100 {
            assert_eq!(exp_inv_alpha(&a), a.pow(INV_ALPHA));
            a += a;
        }
    }
}