//! Additive round constants implementation for Anemoi

use super::Felt;
use super::MontFp;
use super::NUM_HASH_ROUNDS;

/// Additive round constants C for Anemoi.
pub(crate) const C: [Felt; NUM_HASH_ROUNDS] = [
    MontFp!("47"),
    MontFp!("23651892696017635051311932573142498414372751183836160853812929249073212051004004868225625270519161718897739063636"),
    MontFp!("108408258685061606189190548723359824672284631663840600361290557059934237530248648468547776202467131425426693074558"),
    MontFp!("115749210796626156355961226217033697648277624267646303812115651330611822636154890997899312710171050197066310511132"),
    MontFp!("32938029009009707750853639216879896521914480482174343926756027817470420463936977428079379763376648137928269263125"),
    MontFp!("40575574972692154693371148853310178623706434729855425467727992185929212015207456735505186051322972223631650259106"),
    MontFp!("181370236551256971820074603382020385729591238290079082087028050467599960619119153377372739336731389135391455621185"),
    MontFp!("43019484312139119677732300239972505565370480796924734841398119658532010090928100522923025131851788434306002926824"),
    MontFp!("91385497451290364286542259114099233528467132764723283516863500403146833995007020614131524882440134900979898531653"),
    MontFp!("63004775662779643318102173655073017170292727075458767603881370099079850547916436871825900459279413632435318780398"),
    MontFp!("140920458640164007835215254871999670108377445538836910604183711238527893171467991103178253057721420949480774852003"),
    MontFp!("97549054128025193700769287508862670117558614323004114314976551460288759340041245540004643782590643846886640773979"),
    MontFp!("96278347916619895715195112013245212124830376058690166198388526542220472759579154278221591926318561118086042871593"),
    MontFp!("108612183029105569994399836305634265412031526543127414597379785132151175890324977808793488834990099767147672869617"),
    MontFp!("225362489072979162213512539818406889007424296425244165443169317881510191359314512459646859899596059150368736166327"),
    MontFp!("56209306302926862950778249401173199654174031995242603771256400500058375887590028538029647242328896119454967681297"),
    MontFp!("231031955458715195381799174777044485601854525156631207622376198634130074791526940437745138207899731693002688840585"),
    MontFp!("130396334416753573520063640008051074182383505586500796395072267471485481914049687837400714069836204000678499479921"),
    MontFp!("43744461731850063793495426636478615687451183664941474782844227827058566027345188184873627510317433406629405991252"),
    MontFp!("39112852565965505257663678430205614999394122012896249727015910278164921677729393032506014175555885650268011231397"),
    MontFp!("181760141745270263450952302597582583356286460441009662908027003371308544134649207847611026835102418500428590764103"),
];

/// Additive round constants D for Anemoi.
pub(crate) const D: [Felt; NUM_HASH_ROUNDS] = [
    MontFp!("34488590135062545868087031159319137804852468367321954738651235022229395779778776369995851751943114683258709527804"),
    MontFp!("241056784058908456452437505264885375297275388471500963605224278691364375440315115522423436875172949018721227198476"),
    MontFp!("166414254837295205511720997500605979238818860309869111951877231185566572210928295104013484999516940244744633018384"),
    MontFp!("221224995998311503252989065004206696395905618499919174700053480874899859345247260744923409963597237673800845378610"),
    MontFp!("15240236139302022991924710210167204174406239135202395787653429103544034631065342920402223744391016882010463439208"),
    MontFp!("118785556802603352692131797332067842755552045060955813879117222418041789917960717487175187168878442544474134007696"),
    MontFp!("104678731306568576136377732468893611714303235867680727477565315402472822000312733630333084472763052420664227524357"),
    MontFp!("196625090404219936187893290906369594741850568511266236693303888960666720777928908165608653841931424377254229338613"),
    MontFp!("188750928750600174144513535094165329036134202474701590924813486127699699987918351422800035214637052491016215647916"),
    MontFp!("242933836124461948529393805077322632596005977806939075809454208126944045612569562988649639253056690675718585768430"),
    MontFp!("113841350571939967348181008290644158554092272791791861021003548247206010634092278993637761827105330062261586699353"),
    MontFp!("170001160071451331422034331416602227446581778206602988556438500174473697079904244920369981806390005712988657694743"),
    MontFp!("27775388167882323813951156695181718266232353439035264600740932958768825889022655440985800444714253826679741700014"),
    MontFp!("179437288481093162618660782517684990625690266727262366620146489301080077140367492528934898103191572286096218101754"),
    MontFp!("9732561053822340630454834657805035698785415233071504782133620917125114109010619832003972175650955950782689127574"),
    MontFp!("148585933547442830635512341734141092095404261497505529872602584387568079843280432730904633280934381649392242992819"),
    MontFp!("147173767594257257409532591263085268343561395136659103006358938576260893867594902641437636052538886024389532195926"),
    MontFp!("19952551237155168757469922017252976652307595375962540248919807187336905465548457927691489154629151739841128740409"),
    MontFp!("32712292383316983476072525778439266316073566406960571834187528455325725561866244719543625572312015877260546818665"),
    MontFp!("247303631248225985472220262355686125402130701285465944084215834425258860935862897438145166166788816041825982522567"),
    MontFp!("190784248916126871346568000276690652521377405168440564845893945593212766139530982430891334129029628699201933051638"),
];
