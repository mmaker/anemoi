//! Additive round constants implementation for Anemoi

use super::BigInteger384;
use super::Felt;
use super::{NUM_COLUMNS, NUM_HASH_ROUNDS};

/// Additive round constants C for Anemoi.
pub(crate) const C: [[Felt; NUM_COLUMNS]; NUM_HASH_ROUNDS] = [
    [
        Felt::new(BigInteger384([
            0xb84ebfffffffe409,
            0x9132d225affff177,
            0x77d13b548673857b,
            0x1b6315703a7a08a5,
            0x837227e22da91c38,
            0x00c061dd4813ea92,
        ])),
        Felt::new(BigInteger384([
            0x62a7dbeae2048989,
            0x42c66ca79f589974,
            0x58c464bc7d6cd826,
            0xa011460aa1069de0,
            0xf0ec0bf46bd02ec6,
            0x008e6fe6537e4eb9,
        ])),
        Felt::new(BigInteger384([
            0xebdeaefd8388b209,
            0x88d4576756141bae,
            0xcf4c52d1bbd83a34,
            0xe421dd8784b87cbb,
            0xdbc23f48ca09debf,
            0x001c3d313d38a745,
        ])),
        Felt::new(BigInteger384([
            0x3803450d9cfdbc69,
            0x3878a3edc2ad5883,
            0xf0dbaeecc744cd02,
            0xd98bd714227f0357,
            0xb0e681e1c36d5f65,
            0x01705fe036d1ee96,
        ])),
        Felt::new(BigInteger384([
            0xaa5f3f4441becc8b,
            0xc375d95ddbf507ec,
            0x3893e7322cbb842d,
            0x498ad99429f0a817,
            0x8906de9973824e95,
            0x00790786f8fa5d60,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x74034dac2931d81f,
            0x4aea7b7eb441de41,
            0xc97fc4e0ae77e68f,
            0x46681d97065a56e1,
            0xf428ac080bc359d6,
            0x00c826345c784e0c,
        ])),
        Felt::new(BigInteger384([
            0xbc46d198e31720c4,
            0x5d65d1fd527ab126,
            0x5263a97425b196e9,
            0x3005423f8fad72ed,
            0xf34bd0b23a47a6b6,
            0x01578b9112bd25fa,
        ])),
        Felt::new(BigInteger384([
            0x422a4769ec3970aa,
            0x69849f06f6f1ba5b,
            0x2309242e775ebed0,
            0xf422d6bc14d213ef,
            0xa6f33a938786f674,
            0x00b366f6942e6d33,
        ])),
        Felt::new(BigInteger384([
            0x4f2e158f8e3ed034,
            0x6c26d976b36329ca,
            0x33f8f33f0a6a1d0c,
            0x3fbe0de333d3f5ba,
            0x0ab90b4296559d8f,
            0x0049778c4ab31ab9,
        ])),
        Felt::new(BigInteger384([
            0x3435317fc3b5318a,
            0xd960c1211af2d7e5,
            0x73247f7aad2b12c0,
            0x6dba3344ee492a25,
            0x8e591c94fff2a59a,
            0x01125daff66ab256,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x2ddac3088f724417,
            0xf226a1094d8acd73,
            0x4ad0b2730feb469e,
            0x19a247b2b8d51bf1,
            0x301c54a3233336d8,
            0x00d737ed857c66a8,
        ])),
        Felt::new(BigInteger384([
            0x75e5bc392886bd1b,
            0xbee5efc4015c81b1,
            0x31f3ba709f34c2db,
            0x445a44639814eace,
            0x2dffce8351837c63,
            0x001e61573416d6c7,
        ])),
        Felt::new(BigInteger384([
            0x0f32820642bbabca,
            0x3182c1734b32f124,
            0x00ad7dda528bd938,
            0x5050074d295e5f61,
            0x4a3b216a786a95bb,
            0x01a3484b47c5e615,
        ])),
        Felt::new(BigInteger384([
            0xa63fbebd9d479c93,
            0x1c976269b7c8c03e,
            0x4fbb977a45d9b8b6,
            0x6a7fdb8132d1b6af,
            0x31f9ccd980ed36af,
            0x016a6f236179ee23,
        ])),
        Felt::new(BigInteger384([
            0xb45c29dbbcf22539,
            0x13bc798088ac3482,
            0x15a878fe9c332d82,
            0x29f46484955e1187,
            0xcc729f67618cc455,
            0x0042e3dfdf820fd3,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x8945f5ba9fbec70f,
            0x52db7d35101dfdd6,
            0xe731a332b2f14ace,
            0x84db593a6f33450b,
            0xd52b89331990e227,
            0x012de4b2bbe0a4c8,
        ])),
        Felt::new(BigInteger384([
            0x54fec443e11b4430,
            0x410bb1e31b433883,
            0x3093db33dd6e5a3c,
            0xc095390a9ff8a063,
            0x1acd00e455511067,
            0x01039ddcb3c99415,
        ])),
        Felt::new(BigInteger384([
            0xba0b0baf967ae706,
            0x24135b4afdad6e57,
            0x9716739e918b0c1e,
            0x369bce73969c6ffb,
            0x625541e513f5db9d,
            0x00ac2dbd0421717b,
        ])),
        Felt::new(BigInteger384([
            0x51e2c50a7b4c0a45,
            0x65506ae5f92928f9,
            0x697c7481e33c12c2,
            0x2c7704e85896ed5f,
            0x968c9c0bc8499f23,
            0x0053489c729f45e3,
        ])),
        Felt::new(BigInteger384([
            0x26b8b5a02e9bf694,
            0xf763a89a590f315d,
            0xc8f10aebf8bbd160,
            0xef93b2719142cd4c,
            0x914540590598b2cf,
            0x00f4eb481d8e467c,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0xe7a341ea0ec835f2,
            0x2d8932e901571700,
            0xffeaede791e5906d,
            0x717acae64da66bbe,
            0x249598981f376f6b,
            0x01197fa7f1e24729,
        ])),
        Felt::new(BigInteger384([
            0x50cb1a8519b3d398,
            0x59d4e3e1808b5607,
            0xf9bf3bc4fed90b4b,
            0x676141569b7052de,
            0xfb4325c6bd3a4a15,
            0x01850ff50498d04c,
        ])),
        Felt::new(BigInteger384([
            0xe5f9d793ccc235a4,
            0x78e11026a7e87135,
            0xaa24b4bf5723b412,
            0x750c5d7d2a2e60af,
            0x953b923f452f4d45,
            0x015ecf40b8c13b88,
        ])),
        Felt::new(BigInteger384([
            0xb61f4b0fa0db3aac,
            0x3b7f5b7455102cd8,
            0x9f42c2bd8ba55cf3,
            0x1877b771703c0dcb,
            0x89459553fdcf841b,
            0x014c28e9cf9f7990,
        ])),
        Felt::new(BigInteger384([
            0xc0df28c550e6ecf9,
            0x1a1166c0325d9fe9,
            0x0629c8814e44b13d,
            0xe65b087bc358efcf,
            0xdb0d90d7041e8b46,
            0x0112068ff1256f80,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x6d12470ea50e10cb,
            0xc0fa39947a07445f,
            0xdd8c20fe8f3a9466,
            0x72ff7297dc37f1f6,
            0xcb755de2ec4dc814,
            0x004a97aceb124e10,
        ])),
        Felt::new(BigInteger384([
            0x837007a966e9736f,
            0x702f9ae58da529d0,
            0x882f22215186cd47,
            0x17d41e161b112edf,
            0xa0ada3814d8fb67b,
            0x003f1d37d1c0a085,
        ])),
        Felt::new(BigInteger384([
            0xf76da1d7f6264ed9,
            0xb33155ab37fee2b6,
            0xfeaf5925fda1da1d,
            0xc4b38a7aa671f3b2,
            0x16c24d48107a6e3a,
            0x00668f11249cd34f,
        ])),
        Felt::new(BigInteger384([
            0xe2d3a22232eb5459,
            0x5c4b6147d1dd76d5,
            0xb6194d239f6b50a2,
            0xda8b2488a2a4edaf,
            0xec968d89c41871e7,
            0x004b92f1a65565b5,
        ])),
        Felt::new(BigInteger384([
            0xe7af14c9387bbd01,
            0x035bb1e5a5352874,
            0x2a7b6477891f814f,
            0xd475e1b6e44328dd,
            0xd8ad3e25354d58f2,
            0x018da1790b1bc546,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0xb7c98917bb436a23,
            0x99da2691bff3fbdf,
            0x5f3fa334e1e41bd8,
            0x11db6b8b6a9fe44d,
            0x486490b928f5fcc4,
            0x018fe3625b0c1e62,
        ])),
        Felt::new(BigInteger384([
            0x63dfa984cf4910ba,
            0x8134dbff7f865250,
            0x2a73ae24925c8f61,
            0x9ab6025de9e9f509,
            0xc398ad1a3668478b,
            0x0078a9aee77070fb,
        ])),
        Felt::new(BigInteger384([
            0xa3c9fb1baecfd8a9,
            0x1784ea29bbe1a1dd,
            0x5305fcd01b2031b5,
            0x8d9ff345e1a8fed9,
            0x74a6848411022ef4,
            0x00d67b111df5af3a,
        ])),
        Felt::new(BigInteger384([
            0x9af69febc1371d7a,
            0xa80a210dc10c30e1,
            0xa61e4da56161cd12,
            0xf52e101794d3efa4,
            0x3fa5667288fc2814,
            0x0115f5df5e8c63f0,
        ])),
        Felt::new(BigInteger384([
            0x21a9563b4f17ebdc,
            0x992130d3f3ee056e,
            0xdf6e7081f6255f22,
            0x028df77d4f3d9121,
            0x50f0b594aec9e395,
            0x00d3b0cdc67f548a,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x7ad28a3e5b2eb157,
            0x7a7f1ace81dfb343,
            0x9789ee32bed7853a,
            0x5d23f0c42eeaa1dc,
            0xa23283072c36215e,
            0x00dcd7a958b08881,
        ])),
        Felt::new(BigInteger384([
            0x06389e16214ef8fb,
            0x68f7cd36a2791818,
            0xdd97cce290886028,
            0x96dce3e6df1f11be,
            0x8c5afae58a2d1f49,
            0x010bb308e97fff66,
        ])),
        Felt::new(BigInteger384([
            0xc42afa0ddb569b2e,
            0xff15ab375fc1e1af,
            0x80d7bcde7c388229,
            0x5c7ea486f1766dbd,
            0xe611ffc5dc4e2bd2,
            0x0024e91e727e96d1,
        ])),
        Felt::new(BigInteger384([
            0x2c30d7954a26d801,
            0x1c8449c73be50a46,
            0x42e88efb8080402d,
            0xae0df0e7fe908004,
            0x40f9bca295b7b4d2,
            0x0132e5cb08cafc05,
        ])),
        Felt::new(BigInteger384([
            0xf31c57bbed824890,
            0x9a99fa01b474718a,
            0xea109ce9ef740811,
            0x7e351b0adbd89cc8,
            0xb542901577b749f5,
            0x012df8cb59356b29,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x9265f3ce4d1828fd,
            0xa3304701fa785273,
            0x7891f98014d7c696,
            0x778ac2b26ef114a2,
            0xcb23f2ba9a2b387c,
            0x010de63c1ea5d8cb,
        ])),
        Felt::new(BigInteger384([
            0xfc8080b5469449fd,
            0x7c8738308b780f87,
            0x7e87dd9ce0292876,
            0x8d2d1f63405565e7,
            0x426ce04b8a2c22d2,
            0x0189786c02997aa6,
        ])),
        Felt::new(BigInteger384([
            0x0025f32f1296d0be,
            0x3a2e16f894791f07,
            0x1ac2fca059070710,
            0x18aabbd88026e4c2,
            0xc79e6cc79d81635a,
            0x0081e9b76ef611fa,
        ])),
        Felt::new(BigInteger384([
            0x6abb8e08bb8530e9,
            0x52ffa57a0153b8d2,
            0x0943d0dfd9877226,
            0xbd7faed1a81db432,
            0x91055a4e8f7b6f75,
            0x0056f911d0a1f5cf,
        ])),
        Felt::new(BigInteger384([
            0x37a5c852e1c86c2d,
            0x8b608af46423e7a5,
            0x1f6b4cd48713773b,
            0xb73c3c495f49b5b8,
            0x7152428be993edae,
            0x009b518d5e89aecf,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x80290e04b1e39d4c,
            0x09a6c9235fa69a1e,
            0x54604b7492a31071,
            0x80610c382ce4b57a,
            0x04889e6b59a208c6,
            0x01420455a4b8acbf,
        ])),
        Felt::new(BigInteger384([
            0x194aecbc78ebdbc8,
            0x67158f577e635adb,
            0xd6ce1b951543bc80,
            0x5fd70c913fc87b0e,
            0x20d58e467c85420a,
            0x017532ba31894464,
        ])),
        Felt::new(BigInteger384([
            0x500d3f26c182ae51,
            0x32954c8937d8806d,
            0x3560895c8ec03b1b,
            0x251a327be8d144c3,
            0x4d3201b47656441a,
            0x01aceb4d08c8f63c,
        ])),
        Felt::new(BigInteger384([
            0xf09cd0f6688d0bd2,
            0x881a3cdabe2027cd,
            0x5923251eb9ebd4f0,
            0x789ad782a157c0b0,
            0xbabbc6d3b66b5ef8,
            0x00657bf485204de5,
        ])),
        Felt::new(BigInteger384([
            0xe69423d6a9e7926c,
            0xeb9d322c84308283,
            0x3e80b220e02ff4d0,
            0xd62502ce219948be,
            0xe0d896043d4db020,
            0x013eb72e18de0ae9,
        ])),
    ],
];
/// Additive round constants D for Anemoi.
pub(crate) const D: [[Felt; NUM_COLUMNS]; NUM_HASH_ROUNDS] = [
    [
        Felt::new(BigInteger384([
            0x0f2b9dddddddc1dd,
            0xbee4d384e77768ea,
            0x022b94b0d25c368b,
            0x3ba4d123a87f69cc,
            0x01b45588a882d235,
            0x013c894bd5041075,
        ])),
        Felt::new(BigInteger384([
            0xc127fdd80251fe53,
            0x5d74ae3de14a3902,
            0xd2a54ef75ccc8895,
            0x201a93c949443fd8,
            0x87b0ceec0cfa8488,
            0x0046ad8fa9d396a3,
        ])),
        Felt::new(BigInteger384([
            0xed09876a3caf6902,
            0x564dfd3dd020e708,
            0x7d66a6d2fc1ecae0,
            0x626f40294e51edff,
            0x8c015712e4c69fd1,
            0x015201294e37fbc8,
        ])),
        Felt::new(BigInteger384([
            0x55aa99a6f100efc3,
            0xdc5e5f9a5f34f54f,
            0xf9065efc602f0192,
            0x173a74e006bf1e21,
            0x3cfb680c7e692b4b,
            0x00d94f731fefdb15,
        ])),
        Felt::new(BigInteger384([
            0xbad0b70a4ed9e6b5,
            0x278aa88f5135f718,
            0x40fe5e7534714b22,
            0x1533b0ad03389c18,
            0x3a00fad89ce6ec14,
            0x002b1eef864ff2f2,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x657b5e0b56e4d2b7,
            0x933663e5e80640f4,
            0x86c2454201fd11ed,
            0xcf83ff8bc23367ca,
            0x9a8a7677376b0d32,
            0x0040371c5113783f,
        ])),
        Felt::new(BigInteger384([
            0xb56226075339b252,
            0x92adfa9b90b93bf5,
            0xff2cbab40cadc1a6,
            0x18e8b63f85bec4a7,
            0xb23030728c3ff9d7,
            0x000bb2b3d0bd7234,
        ])),
        Felt::new(BigInteger384([
            0xddf05257f5354467,
            0x51982be56d4b70f5,
            0x040b9f34bf41c9ca,
            0xdb4a5f9f2c3f34f5,
            0x7f51ef265311b4e5,
            0x00e514680cd8c606,
        ])),
        Felt::new(BigInteger384([
            0x8c795caa32172053,
            0x41b1d96f7c37b1d7,
            0x8dff2c8364fa13eb,
            0x0069abe366dcd3d5,
            0x852893f66ec0b00f,
            0x005c8adeb3411c72,
        ])),
        Felt::new(BigInteger384([
            0x644a9bc720a56879,
            0x6f1ad49ebc80b252,
            0xcd6a7ff276869c03,
            0xbc600a921659e177,
            0x2daddb5d46c689b3,
            0x016e98d803305d23,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0xd324015401688047,
            0xfb9be3cde7a0ef39,
            0x4fb39a9a5b7c56a1,
            0x1c3e01dc0c9f57b3,
            0xdf6560638adc3d36,
            0x01290b3a4c5237af,
        ])),
        Felt::new(BigInteger384([
            0xa7dafe93dcec9042,
            0xcc62d003d5eccb94,
            0x455095a638461a3d,
            0xc0e06a8b270c7af1,
            0xbc0675554c1e6bc0,
            0x015a8524dc16dac0,
        ])),
        Felt::new(BigInteger384([
            0xd9c0fae08ffac11e,
            0xc3b44b6af7de66d1,
            0x0a5cfe76d87180d6,
            0x96d48e71d7c797b1,
            0x6546118e13555df2,
            0x01007ddb7ae5d4d2,
        ])),
        Felt::new(BigInteger384([
            0x125373c485632e49,
            0x9c405f7bb6ef075f,
            0xd26ed654de6c4c39,
            0x8a8877c2fcd6ac14,
            0xef15911e28b852f5,
            0x00a90a94847d85c6,
        ])),
        Felt::new(BigInteger384([
            0x9842c20f5e259dc0,
            0x6a9fe75b908bce03,
            0xb78ee13c5d9a9b69,
            0xf21a1406555ff3b2,
            0x74ae9f80e461fb6f,
            0x0178e16cbe826175,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x60b0e3af020e9c96,
            0x416a5b22dfa65319,
            0x630bfe8ce801f965,
            0x9deb2a4a31ecee30,
            0x5f0972a6468933bb,
            0x00b72055d7a051b9,
        ])),
        Felt::new(BigInteger384([
            0xb915b64785dab0ae,
            0x33a22d4c2545b5e2,
            0xbae8299c5fff5032,
            0x538f76189ddf9de8,
            0x83688569153b4afb,
            0x01772a00b0b373f7,
        ])),
        Felt::new(BigInteger384([
            0x3bc3f432d41395b2,
            0xb269ddb00fcb1782,
            0x36b0c99dbaf99a50,
            0xadb74671b4ea293d,
            0x1e30157be0d13845,
            0x00ef05e9a3f04d0c,
        ])),
        Felt::new(BigInteger384([
            0x7520e9ba53c13553,
            0xe11e60655dc1a396,
            0x821a88bf1f578cd9,
            0x7d169203928063b6,
            0xf47843c3a2054fda,
            0x007786aa0251ca5a,
        ])),
        Felt::new(BigInteger384([
            0x3cc0fd7cc0290872,
            0x3360b19e9660fe5a,
            0xe1cee65ca3a2dddc,
            0xce2d78d9c0341cda,
            0x14161e254dbd3520,
            0x0162512b51787407,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0xca0db9d8c5bb8e8a,
            0x89ba58de009b21e4,
            0x76b3390706f68af1,
            0x94a05f70acd10464,
            0x8c707d625b7e47d7,
            0x00134adace20802d,
        ])),
        Felt::new(BigInteger384([
            0xbfe196831316c327,
            0xba0da751ba498907,
            0x7f0179f2c16a4d2e,
            0x047141df35c83fe5,
            0x41dba5a28c730b81,
            0x01692ba8c2013c43,
        ])),
        Felt::new(BigInteger384([
            0x72b24a115efe6761,
            0x74d9da92e9c1d001,
            0x44acfa83c0928e32,
            0xf63d98f5e4ed0972,
            0x2f13612d215930c5,
            0x011236fd190ea32d,
        ])),
        Felt::new(BigInteger384([
            0xe45cf9b9cdf3e8cb,
            0x24ef98fae9645d16,
            0xb2cec6c007c122f8,
            0x732d0807469673a3,
            0xc52e3862e6d9bbaa,
            0x00e0f6871fd08a1b,
        ])),
        Felt::new(BigInteger384([
            0xe1e6fa9c371781e8,
            0xc3b0b7cb9f6b2287,
            0x19f593b7392c09a5,
            0xcf0a925e8ebb2ede,
            0x3bdb69fa5b91946f,
            0x00effc02e58e291f,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0xa849acfcabce741c,
            0x7a896a113fb51232,
            0x37a73d1ddfbabc9c,
            0x37f5cc90da6a5ee9,
            0xf7eeebcf560417d7,
            0x007a7516abf0be83,
        ])),
        Felt::new(BigInteger384([
            0x4b5371a6b0196db7,
            0x2dc668dd8dcd1fc0,
            0xf0c4314eef873cdc,
            0x56b4e40d5470f032,
            0xabe4cc7f4a37ef3d,
            0x01594b2273c943ea,
        ])),
        Felt::new(BigInteger384([
            0xdcf30254d82f8b4f,
            0x0c882a9f40420471,
            0x7c8a6fea427fe1ef,
            0xe7b58b62003870c2,
            0x7538c5581a13c911,
            0x01500904698a7262,
        ])),
        Felt::new(BigInteger384([
            0x69de3ecbafd10d31,
            0xa319a9562c9b6a03,
            0xacf82225f6f64458,
            0xd7113a8d180727d4,
            0xed1dd9bada9220cd,
            0x011672c5db26adaf,
        ])),
        Felt::new(BigInteger384([
            0xdc7b149f6e795ca8,
            0xf34db034a8ac6e01,
            0x02a69e7d956cbf68,
            0x44d357154db828aa,
            0x37debaaa4d8e9037,
            0x00f36edccc5fa569,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x0968c0f83b99d274,
            0xf4e19a4a70777337,
            0x9e9a00528b97de76,
            0x1080aec869a53647,
            0x67b02dea6c872871,
            0x00af1aa68349a6c1,
        ])),
        Felt::new(BigInteger384([
            0x422ae574920f1002,
            0xe043ed336a83f1c4,
            0x7847fe508990995e,
            0x1345b199241c9b64,
            0xc1a1e55d0ceb5c38,
            0x00823173f0d82c4c,
        ])),
        Felt::new(BigInteger384([
            0x9fb72d8b0a6f1a1f,
            0x12540259aefa6d1c,
            0xb6205492b931d3ef,
            0xea50dd713c4260f0,
            0xc5ef0bd8f47665b5,
            0x00af4edeca426639,
        ])),
        Felt::new(BigInteger384([
            0x38690e87b7b2db52,
            0x9050ac58069fcd93,
            0x823c63a612205b31,
            0x2b630f600b090ed1,
            0x32fec1e87950b2e5,
            0x00d02f8dfabcc3d6,
        ])),
        Felt::new(BigInteger384([
            0xb1e5e803feab9084,
            0x4196cfa3123af47f,
            0xbbcc4db615af7fa5,
            0xc6bd3012ba7a8985,
            0x692f471f0d873ffe,
            0x00d7125206e75d83,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x8da057eab75619e6,
            0x3db569dc13d3f23f,
            0x6528030b37ada2fc,
            0xe5b854fca6068f63,
            0x809575400013a9f9,
            0x017db231029e1f9c,
        ])),
        Felt::new(BigInteger384([
            0x20a9afd1bfe5f880,
            0x192a5c7b3ee77f30,
            0x9abc72999cd57d49,
            0x7f38da2a90734017,
            0x8340826f845b47a9,
            0x00e8a3cb5cd2b888,
        ])),
        Felt::new(BigInteger384([
            0x8146c24912c6dce2,
            0x62139ebc344b7493,
            0x7235cc5be96c7f87,
            0x431eafadc4266b61,
            0xf671dc22500ebf82,
            0x017f602fa07b5c8c,
        ])),
        Felt::new(BigInteger384([
            0x05c91bfd1c739616,
            0x55ee532232e96e9c,
            0x8e56fa874657e16f,
            0x540f3738ebe7272e,
            0x2d2f675fa9b75356,
            0x00c088770ee659bc,
        ])),
        Felt::new(BigInteger384([
            0xbf7ebf5078e6ed75,
            0x943316e184322840,
            0x35becfa924173bb7,
            0xb2309aa8be371d2a,
            0xc65d70e6fa1fba12,
            0x0104c34d038871f3,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0xf1174c8186008859,
            0x4155cc8eea9fc380,
            0x77e6c82a1d0f8cf7,
            0xb5119d96ed8f8a7c,
            0x07ff78ff185fa7e3,
            0x002f74028c46bb3c,
        ])),
        Felt::new(BigInteger384([
            0xe7dddd77c1ec4050,
            0x1eb45b38b619a8b1,
            0x8c569f5535e13636,
            0x449e6645fa213022,
            0x5e0601a19b527b39,
            0x019556b351649008,
        ])),
        Felt::new(BigInteger384([
            0x0925467126c8093f,
            0x781b40fcc735e3fc,
            0x3dd7c5ef559cad0c,
            0xb43d3dab5a596ab9,
            0x3676dd2fbb98ddd5,
            0x005d140760a6230b,
        ])),
        Felt::new(BigInteger384([
            0x15401d776a92e5cc,
            0x7e644298868b4f3a,
            0xa55c586ce8ca0407,
            0x329645c19debf73e,
            0xa1ee9ed7ba733e00,
            0x00138942b235afc6,
        ])),
        Felt::new(BigInteger384([
            0xd4f47aee49ee07e0,
            0x76f43b97c214d06c,
            0xbbc39b9505219b80,
            0xba4d0c864a1fd1fb,
            0xa720bd2982f48dd2,
            0x00a10993e45511d9,
        ])),
    ],
    [
        Felt::new(BigInteger384([
            0x6e9da662fd560b87,
            0xba8311b6267c6301,
            0x6151af9f6b285f0b,
            0x617868fc51456f65,
            0x4def95f0b53088a5,
            0x0121d8cf25e2d67c,
        ])),
        Felt::new(BigInteger384([
            0x0f62c92a06cde0f9,
            0x04ee18214fb34bda,
            0xd346109e81400a7a,
            0xa0b5fb609e6175cb,
            0x82bf1b1cfe6461ad,
            0x00911d6e7c189028,
        ])),
        Felt::new(BigInteger384([
            0x63c71213e83df5b0,
            0x6c2ddc4f11439d37,
            0x471e85fca19a2151,
            0x4a1a5c3b67d0fb3c,
            0x025add9d052685d2,
            0x00982209f63d3daf,
        ])),
        Felt::new(BigInteger384([
            0x2ae4a0102a24cf94,
            0xc6359cff1a06160b,
            0x02d8422c997bef0b,
            0x9141f0523ce847ce,
            0xd8307c9dbebd3dfa,
            0x00e052d87a3d4f29,
        ])),
        Felt::new(BigInteger384([
            0x8e9d561d24973cfd,
            0xd2dc489188cfc320,
            0xc98234327482594f,
            0x62a37af7b13c9583,
            0x5cf77c2247671781,
            0x00547ba19a6da456,
        ])),
    ],
];
