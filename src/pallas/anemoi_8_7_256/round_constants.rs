use super::BigInteger256;
use super::Felt;
use super::{NUM_COLUMNS, NUM_HASH_ROUNDS};

/// Additive round constants C for Anemoi.
pub(crate) const C: [[Felt; NUM_COLUMNS]; NUM_HASH_ROUNDS] = [
    [
        Felt::new(BigInteger256([
            0x0b0ce7e8ffffff6d,
            0x51762746a8ccf527,
            0xffffffffffffffec,
            0x3fffffffffffffff,
        ])),
        Felt::new(BigInteger256([
            0x6ddbc2775ae1e64e,
            0x493532387365743b,
            0x7465ba349a3d7dff,
            0x369d2381db9addac,
        ])),
        Felt::new(BigInteger256([
            0x1b266cc80424b9a1,
            0x356a97de777163a4,
            0x187f5a9fe8f98d55,
            0x12e9a616af089923,
        ])),
        Felt::new(BigInteger256([
            0x6edf278b3961df33,
            0x38929a969899cbae,
            0xdaad4be861430678,
            0x1ae22589d785c779,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xc06782695ec9eb24,
            0x890d38c8c464d4d5,
            0x1a4caa8fbcd5d19d,
            0x35723d110821bec9,
        ])),
        Felt::new(BigInteger256([
            0x165b79212d34a3ec,
            0xe10a3f22e4d42e69,
            0xa0630790f9a6117a,
            0x12ce77b5ad7bbbd3,
        ])),
        Felt::new(BigInteger256([
            0x425cde59128a5fcc,
            0xc3fd151a12906631,
            0xec9a68079d632be7,
            0x256e0771846c78a8,
        ])),
        Felt::new(BigInteger256([
            0xfc1d060970aa4315,
            0x4567278e0cec1f91,
            0xe23a5bcf078d5feb,
            0x21942c0c6cef935b,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x78817653ff6be0d9,
            0x81a69f4d7f450cd7,
            0xd744923c361f9b6a,
            0x0afbbe8518033770,
        ])),
        Felt::new(BigInteger256([
            0xa2c54cb003b36008,
            0xf03315b51a65867b,
            0xa6fa500abcc0d495,
            0x083709719e297528,
        ])),
        Felt::new(BigInteger256([
            0xc41a0f0888ba6038,
            0x3c71c5e23ddb64d9,
            0xdac986030093c504,
            0x390797e35437c685,
        ])),
        Felt::new(BigInteger256([
            0xe38ff1cdad4465b7,
            0x81ff4df99fd23fd0,
            0x7164a4990355e201,
            0x3a3c6f48bbe66253,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x5fcd2e48568e5ee3,
            0xc8b7bcea8555fcb7,
            0xa4d4f6d10e4528a1,
            0x2b59f32a502b98e5,
        ])),
        Felt::new(BigInteger256([
            0xbf69aa170d574683,
            0x91e711681a5eb680,
            0x3c339c0bfbed0d17,
            0x3193aa5fa71a77f1,
        ])),
        Felt::new(BigInteger256([
            0x9033705906b190f8,
            0x3b605cc01a6fec6f,
            0x266ead4642c9b6c3,
            0x071a6e88c1f81430,
        ])),
        Felt::new(BigInteger256([
            0xcd7e1aa4e683395f,
            0xda08124a25b5f693,
            0xe6557faf7a62b974,
            0x23509a172ae67ebf,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x2c2ca7a82f04be0d,
            0x7ea536487a11888f,
            0xdee482ac9661839a,
            0x3468093968a345e1,
        ])),
        Felt::new(BigInteger256([
            0xd2fe7dce74443346,
            0xb30efbc93610eabf,
            0xf3155ded0566d50f,
            0x088a5994bca78d32,
        ])),
        Felt::new(BigInteger256([
            0x2746e6470d504a2c,
            0x8a685a2d13350e17,
            0x8855e741038c229a,
            0x052adbefa560d7a4,
        ])),
        Felt::new(BigInteger256([
            0x6e9bfc64c1975d38,
            0x7043c5d20b7375c2,
            0xf5297b2df7da0da4,
            0x1cd07fd07501ab2a,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x871637143fb75990,
            0x25b881420f49c51d,
            0xf4415b05ce4edae9,
            0x2a07d022621ddc85,
        ])),
        Felt::new(BigInteger256([
            0xc0e464fb690630dc,
            0xed16fe1fc419524f,
            0xad6614601ef6535c,
            0x35de172d4f116cdf,
        ])),
        Felt::new(BigInteger256([
            0xaa46b62b928e0920,
            0x5f46d1e952984763,
            0x7ac518928615751c,
            0x32b94045d3a69311,
        ])),
        Felt::new(BigInteger256([
            0x4999b7b19aec5eee,
            0x88918e785ca8d85b,
            0xeff2ad6170c772af,
            0x18fbc9adc7d9dead,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xc57dd1a29ee6d139,
            0xe53d649c324f4d80,
            0x9accce2be6a38f3d,
            0x2f74d48d03e8cafa,
        ])),
        Felt::new(BigInteger256([
            0x29202687d61ada16,
            0x0b9779403bccf723,
            0x2509f4ec4dd24e43,
            0x1aa949d8e3a9149f,
        ])),
        Felt::new(BigInteger256([
            0x2ee6caeebc548874,
            0x104d4a7dca1069e8,
            0xa7e22d1091d69d5a,
            0x2162d2798391bdd2,
        ])),
        Felt::new(BigInteger256([
            0x423d5c93e505a378,
            0x04298722ad313d62,
            0x32b349549509754e,
            0x2d54cd0ede33dcb7,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xbbaf94bfa7d60018,
            0xe3a245502d9cc2e1,
            0x6caedc363b2c54f2,
            0x28d40806dcb2e31c,
        ])),
        Felt::new(BigInteger256([
            0x864a284d5e2259a1,
            0xf9c42416cf2acd71,
            0x1586def4a90ccd5f,
            0x07619650c37c8d1b,
        ])),
        Felt::new(BigInteger256([
            0x57896c725f627cea,
            0x1c3a9f34eea5da5f,
            0x357f367f4f38140b,
            0x228b2d95abb1a022,
        ])),
        Felt::new(BigInteger256([
            0x97d58865c5708635,
            0x323bdc03d7bda9c3,
            0x05503b7de48d8641,
            0x3acfa8fdd097d486,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x5de7b12757bc3840,
            0x2f9a37d2ba0bdbc0,
            0xfefe6f5ded23785f,
            0x356ca8ded5aca8db,
        ])),
        Felt::new(BigInteger256([
            0x3ba0ab7b46a71ce3,
            0x71c57787a14ddc6e,
            0xba527b4c0afaf6a0,
            0x265a122f91c4cfdc,
        ])),
        Felt::new(BigInteger256([
            0x2b6f76977ba1b4e8,
            0x10ccf9a8e833798d,
            0x8f356939570f76a7,
            0x1a1c7a6497e002d2,
        ])),
        Felt::new(BigInteger256([
            0x71222773f8d815e9,
            0x511a2970cbac88fb,
            0xc0b045e6f7cedd25,
            0x278803eb7c4084a9,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xa9bbf8f179ce7b75,
            0x66364a9c18f0f5c0,
            0xca28f26ed2a5879f,
            0x1f35a427ea54c3dc,
        ])),
        Felt::new(BigInteger256([
            0x5258790cccc1edc7,
            0x0d318a1213a25022,
            0xb8549cd96df4a31c,
            0x3beccdd711ae91e7,
        ])),
        Felt::new(BigInteger256([
            0x7d1b38567bf65b7c,
            0x9a40c209709f7816,
            0x0a6b80b2c8ee4621,
            0x31e06bbd8a5d74bd,
        ])),
        Felt::new(BigInteger256([
            0x372314e58655f869,
            0xc1ecdfc65f4ffbf0,
            0x765057c8c7c70978,
            0x19fc150f7a6d7cbd,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xbec9bc4e0c2aab74,
            0xbb7e3d50de386c01,
            0x63b2e85f530b50bc,
            0x0a9a04ef51428741,
        ])),
        Felt::new(BigInteger256([
            0x8f123ae1c5549a31,
            0x0b0a29d0ebd55b3b,
            0xd5d11c742ac52fb7,
            0x22106b94f916756b,
        ])),
        Felt::new(BigInteger256([
            0x383fffb72b0563a3,
            0x72d7ff6b7c039cf8,
            0xb40aedcb1cd95f29,
            0x335b376e38357ae7,
        ])),
        Felt::new(BigInteger256([
            0xb918acde4a02a022,
            0x3605715b53d78fcc,
            0x99e4cba806802e66,
            0x3fe7e7ded3c1931b,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xee4c95b6874f8b47,
            0x405d57fe5ace482b,
            0xf52b6381e64c9f0c,
            0x1e507aa3934360ae,
        ])),
        Felt::new(BigInteger256([
            0xddfce35ef1930fb0,
            0x86d4885f84401a4f,
            0xcd157d73831ae8b7,
            0x2a51345b7ad83aa6,
        ])),
        Felt::new(BigInteger256([
            0x5a7e715bdaa019de,
            0xb44f6c4494fdd674,
            0xd80450b1eb80677c,
            0x2936e57a9194421c,
        ])),
        Felt::new(BigInteger256([
            0xbaeb45d916573004,
            0xbf43ad089ccc4226,
            0x3512a9b953218690,
            0x29599fa045858042,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x4a460bbab9a500ba,
            0x13a2234c1e8f0b56,
            0x5502cdc2d0b4c33a,
            0x381de7018d672a67,
        ])),
        Felt::new(BigInteger256([
            0xde3350fb3e7bd38d,
            0x91e2bfd299da1a3d,
            0xaf2dbe09fa59b2d3,
            0x1356b7042e029abf,
        ])),
        Felt::new(BigInteger256([
            0x4d6de280bd49701d,
            0xc3264a7b56a846a2,
            0x81006d8cd9518274,
            0x097b7c0e015fa8af,
        ])),
        Felt::new(BigInteger256([
            0x2cff9e943c68888a,
            0x0a0ed8aeba7d6cb2,
            0xc9e9007196b53a32,
            0x1ba914fc4031f8d1,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xf547489dc4b356a8,
            0x6847f2699624ecf4,
            0x4e4301673ee23b80,
            0x29af5acf6f779772,
        ])),
        Felt::new(BigInteger256([
            0xb59bf7586a217691,
            0xb6b5e8a5f3502454,
            0x76c4cd32c6ab3a38,
            0x0c035cf6501f4ab6,
        ])),
        Felt::new(BigInteger256([
            0x6652ff8b5c3dfab9,
            0xaef3c4aee245d6c5,
            0x72a2b6585b488945,
            0x3f80c3f04b1cf39c,
        ])),
        Felt::new(BigInteger256([
            0xc5c5a24fc62cd3ba,
            0x4408255b075f5568,
            0xd7f0bca12cfb2975,
            0x1f620e8506d0e001,
        ])),
    ],
];

/// Additive round constants D for Anemoi.
pub(crate) const D: [[Felt; NUM_COLUMNS]; NUM_HASH_ROUNDS] = [
    [
        Felt::new(BigInteger256([
            0x7c5e333a99999905,
            0xe76b98e699eb6694,
            0xccccccccccccccb8,
            0x0ccccccccccccccc,
        ])),
        Felt::new(BigInteger256([
            0xedd3988924d2d2b4,
            0x179cf9e9cc56ac29,
            0x66ae0597b58a9ce5,
            0x25318a7374ca34ba,
        ])),
        Felt::new(BigInteger256([
            0xf167898d36560552,
            0x9f4a81decc303423,
            0x746b46e45e44d47e,
            0x110e731576f5d679,
        ])),
        Felt::new(BigInteger256([
            0x4224a2b56447c8bf,
            0x1ddd9e11aa8d7844,
            0x8f304cc097fa4c80,
            0x075c7c255b4c718e,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xb538a036662d31cf,
            0x57c1e2e37215e9f1,
            0xd5b55175ecad2084,
            0x3fcb8bcd293285dc,
        ])),
        Felt::new(BigInteger256([
            0x19d321ae64ef3d65,
            0xe8313f4efa580a06,
            0x81472d0d77fdb27a,
            0x3eef60969aef0d28,
        ])),
        Felt::new(BigInteger256([
            0x02f09cacb285588f,
            0x44559e991a94e144,
            0x37222e6575b8f52b,
            0x211f565fa09db046,
        ])),
        Felt::new(BigInteger256([
            0xb9b522c20959d9b3,
            0x412aca87d22576ba,
            0x855936c0a14f280d,
            0x0b9b049744fa37b7,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xbc72050c22d9a9e8,
            0x30914f1fcdfa2528,
            0x2a125fb3f317ea02,
            0x096c722acbf6dff1,
        ])),
        Felt::new(BigInteger256([
            0xf55c662857787be5,
            0xd7901b98d0ed654d,
            0x1f439c18c8397546,
            0x286f573c1e7fa7ea,
        ])),
        Felt::new(BigInteger256([
            0xd3cd3e4744bfdb5f,
            0x9d005518e6e3e321,
            0xbcb672f2660a8df8,
            0x28d04bbb034bdf8f,
        ])),
        Felt::new(BigInteger256([
            0xf0477f7161fe7eb9,
            0x5df8f6ab060f9a2e,
            0xabe8a61c2a38a9d4,
            0x185aacbd26d3e81b,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x0b1e5a280780e514,
            0x1c94d4abe2160df4,
            0xc0756ba23b24a120,
            0x25826c054c39a181,
        ])),
        Felt::new(BigInteger256([
            0xe0342fc9eea11f81,
            0xfbefe63ed5a49522,
            0x7d4f8f73774cd7ae,
            0x0d83bd5f6f8b0ace,
        ])),
        Felt::new(BigInteger256([
            0xa0746dac503bc942,
            0x6327ece1dad05cbe,
            0xd12e418f1827a99e,
            0x329ae795b9268d55,
        ])),
        Felt::new(BigInteger256([
            0xdac3765d28c20f84,
            0x7d3abbe6a34b42f8,
            0xe9ac288c112cab2e,
            0x3d269cc0ddee64a3,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x3f8b840d9eb3db04,
            0x087ca030fa92a8ea,
            0xf882c9b9af688615,
            0x37aa6a3ad71e7af3,
        ])),
        Felt::new(BigInteger256([
            0xf503e4f4144aa30b,
            0x7558bbc31e64d19b,
            0x322f23906cee29a3,
            0x2d9454baf7854c86,
        ])),
        Felt::new(BigInteger256([
            0x9f9594201597193c,
            0xe82a3c75f7568d84,
            0x31134dc5c5119f71,
            0x39c53d230efc7d40,
        ])),
        Felt::new(BigInteger256([
            0xe3ef08a2c292ca23,
            0x4970c195acc9d145,
            0xf67df6467acb895a,
            0x3fc06aa09a76bd84,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xda46a62b8bf193f7,
            0x40c8dc4e83587830,
            0x75410cb54cdeb781,
            0x0db0365e854b71c3,
        ])),
        Felt::new(BigInteger256([
            0x22bb5ed2e597be11,
            0x4099af3d9ffacbe4,
            0x53e144a5ec06820e,
            0x3b4e178e3ea18c5e,
        ])),
        Felt::new(BigInteger256([
            0xc939c5c9775ff59f,
            0x2bfb0c5a20fa606d,
            0x8ae3e9b9ad23cc11,
            0x07b9a6b3f1f498d8,
        ])),
        Felt::new(BigInteger256([
            0xfebe56a17872e949,
            0xf2f77b5ff18cc696,
            0x58a8931c5941c882,
            0x1c51b9b8a2015133,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xbe7754780505a69c,
            0x275a45ac4d2c4d8a,
            0xda0391063147fa17,
            0x04c275ae4e74f53c,
        ])),
        Felt::new(BigInteger256([
            0x30c0341d6c910247,
            0x8626b061be7cbdae,
            0x89bc365ce6f70b35,
            0x11be851efa97c922,
        ])),
        Felt::new(BigInteger256([
            0x8cd01f37bb0b0ff0,
            0x2654a3ee488dc904,
            0x76380f6284f98290,
            0x280873ccc93e589e,
        ])),
        Felt::new(BigInteger256([
            0x9d2b0f41dc70c8cf,
            0x959bfa0de8e37894,
            0x59a0403a49985962,
            0x224ff7fedfb9e441,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xc78b31f8b14956fa,
            0x181eafd6d8230436,
            0x0e7b8e6fa055b802,
            0x3f18de1c0ffb6edd,
        ])),
        Felt::new(BigInteger256([
            0xa0cc504697ed0351,
            0x66b2e4aee183d547,
            0xdccf0fc45cb68288,
            0x3f6e068ac327a31c,
        ])),
        Felt::new(BigInteger256([
            0x2f27aa32016d85e4,
            0x025ae91ff37f81ab,
            0x666b08305cdff177,
            0x2a2803dcda1a9c6c,
        ])),
        Felt::new(BigInteger256([
            0x6c78248a60302d0a,
            0x93c73f6999cc2d25,
            0x8ed321c2b3a1628b,
            0x30c208e1bada3d8e,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xbd841ce928afa16d,
            0xf7cd0804568a48a8,
            0x0e28adfb86eb08b3,
            0x327949c8522dd1f8,
        ])),
        Felt::new(BigInteger256([
            0x10b6711047f1d8dd,
            0x502404ce9c5216bc,
            0xeef8387ff342d90e,
            0x052e4d3ddaa88339,
        ])),
        Felt::new(BigInteger256([
            0x56ce82dfe52cd02d,
            0x8aa3a93edf054c6c,
            0x2d7ec74e99558158,
            0x08811b800f819c78,
        ])),
        Felt::new(BigInteger256([
            0x998592215b17cf09,
            0x465bf2817fb337f0,
            0xb790b88ffb80e6b5,
            0x04422ea3afbb8b0d,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x36e3791e5d34a163,
            0x3d733720169c243a,
            0x3c7ec3b1bbf0695e,
            0x22cfa289a878e567,
        ])),
        Felt::new(BigInteger256([
            0x54f9530ce07f6682,
            0xfa9a33ab6fd34c01,
            0x5025ecb2a5bfd6f3,
            0x214e665d9c353db3,
        ])),
        Felt::new(BigInteger256([
            0xd6055909f7f43382,
            0x23218df1c89e0c86,
            0x0be0716d5ab7a23d,
            0x26d26a5143a206d1,
        ])),
        Felt::new(BigInteger256([
            0x263ec4eafb086e4b,
            0xe87f5e257dd06592,
            0xd05c5d171afc6472,
            0x3d439d3fef8b7b8f,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x246cdec06c4f8847,
            0xe5d4931a509266e2,
            0xf3c725171c5bded4,
            0x2200026ebde51bf0,
        ])),
        Felt::new(BigInteger256([
            0x6a2eb72755d0c9d1,
            0x4b8c3cafbcb52381,
            0x8b60d7c242960fe8,
            0x1b3e0339321b945c,
        ])),
        Felt::new(BigInteger256([
            0x69a5c2b023c1f28e,
            0x4ed2349948b0fdcf,
            0xd33e49fa8ea8679e,
            0x3c19351f9ff88020,
        ])),
        Felt::new(BigInteger256([
            0xe782ce3c3b73cce8,
            0x8d6ac003ddb9ccb9,
            0x11af3c6b39bb35b9,
            0x36fb6f2cf75e0513,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xc1c726e7dfaf9646,
            0xef5696372d613703,
            0x663f9eb2a6c21ce7,
            0x0f2c86af2a25edb5,
        ])),
        Felt::new(BigInteger256([
            0xc01dff507a4a6d7d,
            0x6e401ca9bea5cfa8,
            0x63a5373a9210b8ac,
            0x3cf4da8bde1d51ee,
        ])),
        Felt::new(BigInteger256([
            0xf9bba313cb97d6f5,
            0x14ec89e1c1e42b42,
            0xd837ab5a54745fb5,
            0x0b6af1b823973fac,
        ])),
        Felt::new(BigInteger256([
            0xf05a06e300038af7,
            0xbd927d1c90346c26,
            0x8ddd18f57d817da7,
            0x39e3357a9361ea90,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x381c59056eddee2e,
            0x3951d7261cc66c09,
            0x26605f1dc5e06373,
            0x2076f42c1b86dab8,
        ])),
        Felt::new(BigInteger256([
            0xdab02906240c13cf,
            0xf004c9bdffe44171,
            0xa606cdfb3e05a525,
            0x1d775e538884d551,
        ])),
        Felt::new(BigInteger256([
            0xa034013f0b1a0faa,
            0xbcc076b5b8800667,
            0xe17d1e5f76fb9d0a,
            0x232c896a8a9fc989,
        ])),
        Felt::new(BigInteger256([
            0x7cca1bb782edc5f2,
            0x7f141e63d98a088d,
            0x82fcc5d7f5cb53a6,
            0x23afabf5854b866a,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x105009cd401507ad,
            0x607201d267ecc82c,
            0x161a7d62ad4161c0,
            0x31d8d28a17c47837,
        ])),
        Felt::new(BigInteger256([
            0xdf4b434815da7a64,
            0xe7524e202ceac60c,
            0x6417c7c4838ab291,
            0x35f46ed5c4ceb5bc,
        ])),
        Felt::new(BigInteger256([
            0x4d1e614170375dd6,
            0x58c1b37c0e6117f3,
            0xc99951cb722629e2,
            0x39023bdcee8a44ea,
        ])),
        Felt::new(BigInteger256([
            0xa995626ad2dad4b2,
            0x69412da2f0af72ac,
            0x877e6ca80544c8f0,
            0x0739100e66179e0e,
        ])),
    ],
];
