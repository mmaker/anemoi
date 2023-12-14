//! Additive round constants implementation for Anemoi

use super::Felt;
use super::MontFp;
use super::NUM_HASH_ROUNDS;

/// Additive round constants C for Anemoi.
pub(crate) const C: [Felt; NUM_HASH_ROUNDS] = [
    MontFp!("34"),
    MontFp!("1802890129851246813659007827792457593591967954225177465003958629059592331820704634471444444986557107601258510809477"),
    MontFp!("1163541058489375705939253000052582717589688304227800166492711905258422639004264764757007766200652553778388947035196"),
    MontFp!("2993830509114556863244789033608536874914179736946552181411992791029556788397323446496586721760740748289013332901133"),
    MontFp!("573867724519177799887804458177387030173270153738844699474012364904682058619398469863390936470658172346803816809223"),
    MontFp!("1133666418013810241414908075022499235296993280312781822452385964216874801671701825960318123895614335938760508544136"),
    MontFp!("1432935248360662927643819144861808734341932605053762049493082292040138104615792396202250808062614715861752190946805"),
    MontFp!("2937843952585528477034428820500164949369595385650147011395924721149432944978203468423932270486030958918618718133774"),
    MontFp!("1119368933669939254933129802114314937366171824793327290199562881033964528457085147370031558362957166251919093672436"),
    MontFp!("3106755410607727733260023252411795216281474922841743919595051448766548339207630174571583789429811949294710997058661"),
    MontFp!("2181777351436712524740845170723304835772975798682434494893533952443015954881544024162116615968295237954156090015479"),
    MontFp!("3594492996608017165923826832404826992424753508505712056193258478167704471457155555827277985636181897273436531670345"),
    MontFp!("1426521663463083722392973856516691592534870899730251620491465314056437420299837766620639706949497870380582190400416"),
    MontFp!("1684777022564509910904192199836013514761083181134622416169381549848716459174831281625863491929688843422335344576677"),
    MontFp!("1688979401138880554981724910222208650996449967947279754292953372512949480970306414036172330216782564955929240409068"),
    MontFp!("3512925353137261348847596912239551258170856178591341017420926788668816864889416645962580775713203582391545663739552"),
    MontFp!("733219075109712156475995688873560365442338801006979215051905331157846842568502156349894827650896682475551119667297"),
    MontFp!("2130054975619973174744403371741304863495489397848123951547954563774815483219602049838760055721033455877301680764746"),
    MontFp!("3435479577321238676106085338022191230460233535736004480469160372248187862402124446994560128042451576677385971092537"),
    MontFp!("3607519736501548892132393317403734982981366738922353566281875214735101274484931343202110112259172453217674205448124"),
    MontFp!("1929643958715127743625944647904392070492359360112367865981681248416264558744555966961262152109680041174837784572887"),
];

/// Additive round constants D for Anemoi.
pub(crate) const D: [Felt; NUM_HASH_ROUNDS] = [
    MontFp!("2001204777610833696708894912867952078278441409969503942666029068062015825245418932221343814564507832018947136279928"),
    MontFp!("1063107713031121519411563272879293251937270524183678010670450933115443400882622227431956230480822501821312815871925"),
    MontFp!("2593616443352813769313984452413363298820096767172941008460934430459434341959084720908601613540992934429124163663031"),
    MontFp!("3192492627653335175744539633470668336196141661930128734747515931998918540002187124742333052074612515566985362129282"),
    MontFp!("1099523866041459732336071250292823203050785916176055703688332369001377317279107674828456376950577717515699602254992"),
    MontFp!("2742942427494406416835851503502765917481859714365711472415318649513714011248170304976608950610168443057107769196062"),
    MontFp!("3720639284256286777885486895808125541131496669581508031047489058058775904278758367623731437121324756978073932189626"),
    MontFp!("3353223398208346765924676129190460622023772300171595814559958111308287910230835659930306488376549488924491887638278"),
    MontFp!("1276711465038890289048882965391056654378440021576268403183802186504675616706971496188786358583532942051011315471748"),
    MontFp!("251914423155492399522837463295128075436204652809902068958033305809317127287025185061274600527694359394022738370358"),
    MontFp!("1934929038618629983185937505823606629077609187460918365721844653106617460736005097993532405601911966181440015801921"),
    MontFp!("2191577806391864262081395743269660031449260656701322427815417216127835312574139344472180342330718235434302798187784"),
    MontFp!("362774526546857925715835949875033233949509299343143435961496795710346570662211776506172330241752626471327042551086"),
    MontFp!("1233347002419474022342137492968527408581274622666052084819463343857616419792012632389960087998811440202381370643937"),
    MontFp!("382538512013452892018920840232549550044647256461871111958654240563054550257906128424618651444161047674708004563884"),
    MontFp!("64827059245538958792055297620581768146599041578429974126202124634797193335844854282977288180473443378637158081429"),
    MontFp!("239562390011794608295943573328660130445846218266088041005894486217503340687510596361609550139436536921838566565546"),
    MontFp!("2776457380714316464672018847603854511186843048916369845741544970082332272178150019025210161601680547308653557620974"),
    MontFp!("2141725340566707395213962950606079509263103328238508974872762593270630773513433438540048756683090186481821008274314"),
    MontFp!("2080623511965151345569175872609238981221507453256384279124917148575326199365229641609239943594215966656255235639886"),
    MontFp!("2276655469155771779674268029314909253225973224611390796231736004192904679578164365992882941278488344041686843569885"),
];
