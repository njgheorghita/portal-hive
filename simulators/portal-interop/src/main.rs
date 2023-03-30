use ethportal_api::jsonrpsee::core::__reexports::serde_json;
use ethportal_api::Discv5ApiClient;
use ethportal_api::{HistoryContentItem, HistoryContentKey, HistoryNetworkApiClient};
use hivesim::{dyn_async, Client, Simulation, Suite, Test, TestSpec, TwoClientTestSpec};
use itertools::Itertools;
use serde_json::json;
use tokio::time::Duration;

const HEADER_WITH_PROOF_KEY: &str =
    "0x006251d65b8a8668efabe2f89c96a5b6332d83b3bbe585089ea6b2ab9b6754f5e9";
const HEADER_WITH_PROOF_VALUE: &str = "0x0800000023020000f90218a00409be8253ad6ac0eb2056bc94194c6ccb83c74f4292c40c82e2dc8203bdc759a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347942a65aca4d5fc5b5c859090a6c34d164135398226a0afbf9bfd23008e8df44a83bb51ade45b993b3253fbce69cf7cec5d628eca6d45a0a7120e4bd136c0b6bdb0fa4990649f8c34d10d180dbd5ad6d03502ae92d32308a0d78aa953fedc7f7c112b2686d0b2b7e37eba716dd1f5d74ef3c8a37005f35215b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000004000000000000000000040000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000860b69dd9d66ce830f424a832fefd88303a68c8456bfb4e398d783010303844765746887676f312e352e31856c696e7578a0e962efb883f91286e4fc6fd12989a70f24c174bd087f472528137c4134af0a1a88e857c5acc15dd82701cead98e305c70563000000000000000000000000000000000000000000000000be1b4a7a57f5316eea09c5e3e349141c46c1cb43664a815d28644cd74f282ca122360456d89447c0d586a8f5490922ea86b20e056879d64d87d104c14c0e594a6d800f67f5331ee2e511dc20e169c644b3df0f4c6b7c1717fc29d4844050b74044b506bf91edd14825aaec4f36fc5ad97b9eed9773aa2df15f80dff21eb668e24d61c29c3fda0fb425078a0479c5ea375ff95ad7780d0cdc87012009fd4a3dd003b06c7a28d6188e6be50ac544548cc7e3ee6cd07a8129f5c6d4d494b62ee8d96d26d0875bc87b56be0bf3e45846c0e3773abfccc239fdab29640b4e2aef297efcc6cb89b00a2566221cb4197ece3f66c24ea89969bd16265a74910aaf08d775116191117416b8799d0984f452a6fba19623442a7f199ef1627f1ae7295963a67db5534a292f98edbfb419ed85756abe76cd2d2bff8eb9b848b1e7b80b8274bbc469a36dce58b48ae57be6312bca843463ac45c54122a9f3fa9dca124b0fd50bce300708549c77b81b031278b9d193464f5e4b14769f6018055a457a577c508e811bcf55b297df3509f3db7e66ec68451e25acfbf935200e246f71e3c48240d00020000000000000000000000000000000000000000000000000000000000000";

const BLOCK_BODY_KEY: &str = "0x012e8097f20e95cbafbc113823a600eb8fc14594758083379badaf18f1dc26ce1b";
const BLOCK_BODY_VALUE: &str = "0x08000000821b00004c000000a20300001e040000d30400008c0900004f0a0000830e0000f50e0000a30f00005110000029110000a31100001d120000551700000c180000c31800007a190000311a0000051b000002f9035201668457ad3fe4851cd25659958304631494881d40237659c251811cec9c364ef91dc08d300c80b902e55f5755290000000000000000000000000000000000000000000000000000000000000080000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec700000000000000000000000000000000000000000000000000000000979aedeb00000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000136f6e65496e6368563446656544796e616d6963000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000979aedeb00000000000000000000000000000000000000000000000011cc8b8cfdb883030000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000002843109459ec64000000000000000000000000f326e4de8f66a0bdc0970b79e0924e33c79f1915000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000c82e95b6c8000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec700000000000000000000000000000000000000000000000000000000979aedeb00000000000000000000000000000000000000000000000011f4c44ef64691ba00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000001c0000000000000003b6d034074c99f3f5331676f6aec2756e1f39b4fc029a83eab4991fe000000000000000000000000000000000000000000000000d4c001a0483403982ac32060b5f72505cef9ad80e0be4ace6e474db4dc958e9742a9c8a89f67af938d037a3c6d902c0369c5e7a6c192dfd60b4cea8089bd23bd08f168c802f87901820436847c41b83e851f398a0fe6826d2294c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2880e92596fd629000084d0e30db0c080a032f695b1360bf53805ed9d2691b8dfb9a8359475a4a0e6f658d3bef18f95bd2aa03b4d36626c574c4314238f72596a0b6c9f25b568282fecf4db4f1e77aa610cef02f8b2018201c68480bf26298522b1f34f9182b5d79495ad61b0a150d79219dcf64e1e6cc01f0b64c4ce80b844095ea7b3000000000000000000000000881d40237659c251811cec9c364ef91dc08d300cffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc001a038a32136c77eb9e81bff5bd620ab3e5efb49fa009039df0ee381463719f93b73a02997a3c639342f56c4093985fb1fcffe22d310ed86ee8a66e8cfad6f06cc833802f904b5018201c7846a330b96851f8a7e38b98304ecd394881d40237659c251811cec9c364ef91dc08d300c80b904455f575529000000000000000000000000000000000000000000000000000000000000008000000000000000000000000095ad61b0a150d79219dcf64e1e6cc01f0b64c4ce000000000000000000000000000000000000000000fe30137375b8c39c8a555700000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000c307846656544796e616d69630000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000036000000000000000000000000095ad61b0a150d79219dcf64e1e6cc01f0b64c4ce000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000000000000000000000000000000000000000000fe30137375b8c39c8a555700000000000000000000000000000000000000000000000000000000bff2873f00000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000001bf2c340000000000000000000000002acf35c9a3f4c5c3f4c78ef5fb64c3ee82f07c4500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000228aa77476c000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4800000000000000000000000095ad61b0a150d79219dcf64e1e6cc01f0b64c4ce00000000000000000000000000000000000000000000000000000000c7a17304000000000000000000000000000000000000000000fe30137375b8c39c8a555700000000000000000000000056178a0d5f301baf6cf3e1cd53d9863437345bf90000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ed6021c55398a3690c2ac3ae45c65decbd36c83d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000627d9b4901ffffffffffffffffffffffffffffffffffffff38758e89627d9ab30000000f0000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000001bba36c2753466094d672305b48ba9f4138ca26324ea598c5bde3b2b6d0186a9841c0bc111cd1d1452e2c40179895bb9ef095003596e89e55a1dc3793129df0e88000000000000000000000000000000000000000000fe30137375b8c39c8a5557869584cd00000000000000000000000011ededebf63bef0ea2d2d071bdf88f71543ec6fb00000000000000000000000000000000000000000000005d39cafba7627d9ab4000000000000000000000000000000000000000000000000b0c080a0b47105e77f8f54501363e1197c88bfb7ad08168457228656085267e9c171bc87a022061ebf3549c12ceb22cf351b5443fdb3ff66822e28641f62d2a538e471d02802f8c00182113e85488e3003c385488e3003c38302896f9444283a0ed172410212762f8dce09e6ea27db830b83e147ecb84d0a0000000033799c715cbac2589a0cc6791a5409ce3547f1f1d00e058c79d0a72c7a5ae802895d5f90b6edbafc870fd348fba2a3d20000000034261d99cef3835800000000000000034fbc5bc2c001a0c40b05baa3d1c7b4e86d7a4558510aca525481b1168318e78e41544251e16c12a0705c682addcb379212870ab04b1a973e4e1fab4a4b0fe10046c700d83a0545d202f904300182a3d685373af8d94885373af8d9488303f56794000000000035b5e5ad9019092c665357240f594e80b8c40000000e9f9076aeb011eeaab8bf0c6de75510128da95498e4b7e67f0000000000000000f79fc43494ce8a4613cb0b2a67a1b1207fd05d27002710000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000004d224452801aced8b2f0aebe155379bb5d594381000000000000000027a463bf7d808f0e000000000000002580b707d5f025b87e000000000000000000000000000000000000000000000000282e06b9a6b590d5f902faf9018394f79fc43494ce8a4613cb0b2a67a1b1207fd05d27f9016ba0136e0edbc21af44a15788a0aa7307a3a81c5300ecdd1b0f03230344d1aeb0406a0136e0edbc21af44a15788a0aa7307a3a81c5300ecdd1b0f03230344d1aeb0405a00000000000000000000000000000000000000000000000000000000000000048a09c04773acff4c5c42718bd0120c72761f458e43068a3961eb935577d1ed4effba00000000000000000000000000000000000000000000000000000000000000001a00000000000000000000000000000000000000000000000000000000000000002a00000000000000000000000000000000000000000000000000000000000000047a0136e0edbc21af44a15788a0aa7307a3a81c5300ecdd1b0f03230344d1aeb0407a0136e0edbc21af44a15788a0aa7307a3a81c5300ecdd1b0f03230344d1aeb0408a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000004f859944d224452801aced8b2f0aebe155379bb5d594381f842a02cd9fc82425a6b359c4bb15ae29636d339e83bcfa49e02ed97ed949ebd2af66ba05ce5caccbd06bf94e383da1e424cdd9ef4c371e1cf5aa91fbed31c4320eba1e2f87a94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0201253027fac026aee58f7b09418e76d7cc44d80dbc60df41fff49090f910d6ca0773a7876937c5ed0f82d77c27cb4373ce23050c0426752349794d61a1fbf51c6a01f064f92372c844ba1cb3c63bf4c654d9a8580b0355025447769b3db4e26968cf89b94b011eeaab8bf0c6de75510128da95498e4b7e67ff884a0000000000000000000000000000000000000000000000000000000000000000ca00000000000000000000000000000000000000000000000000000000000000008a00000000000000000000000000000000000000000000000000000000000000006a0000000000000000000000000000000000000000000000000000000000000000780a096ed4e12cc3eebeba39e5563ff1139617e967125794407a52140a0a76b6d731fa0581fa5d015a9a4eea9eb353e16a44ae4d0c11510409b6a4589e5fd1ff278ae3af87083020778852aa7599fe283015f90944c875e8bd31969f4b753b3ab1611e29f270ba47e880ae53c4a5528c0008025a0cf87b29833f82179a1d3bf30127d9512f392e9ac17375133e0a3ffff05995aa2a0055ee353df5d12f046a2d041b11dffa3d0a166253f5bf05c1264b99b32ed88faf8ac824ae9851e449a9400830186a094dac17f958d2ee523a2206206994597c13d831ec780b844a9059cbb00000000000000000000000088bd4648737098aa9096bfba765dec014d2a11c10000000000000000000000000000000000000000000000000000000010ea71c025a0b7d4735b245fc516206e34396896e30c5c76a76dc4b9e4116342297e5a324ec3a05f1597d8c66e0fadfd6b1bafbf0ad263aed9610f60210c3b78be85df5e816432f8ac824aea851e449a9400830186a094dac17f958d2ee523a2206206994597c13d831ec780b844a9059cbb0000000000000000000000000f893a99b0165d3c92bc7d578afbc2104500761a000000000000000000000000000000000000000000000000000000002f71ff0026a00fc557ecb386c2075114804f913a638f826c379ce6c875f01f0bc74a55a15b82a01a975031836a4bd0f9f84995277c1112f4efe515497897305e5cf03c5497c17202f8d5018303df9b85024d7d6c8085e8d4a51000830129bd94dac17f958d2ee523a2206206994597c13d831ec780b86423b872dd000000000000000000000000b24abf582bab677c3bc8aa60706d212284a35b510000000000000000000000007abe0ce388281d2acf297cb089caef3819b134480000000000000000000000000000000000000000000000000000002fcc3cce80c080a04e00eddba90216b710b07c3725523848b4bf7288cfbbcdc3f84d70fe11c3e36fa01a6cb515d48c3c60b8cebecc6994f5829d6a879c4cbb0de187856eb2c926be8f02f87701831d1e57850241ddf5c085e8d4a5100082f618940329eadd881a8684b20254ccb66c2ae46791e3578808c8dd7dcb7a600080c080a0749657d0c76b979aa9f9c83c2f6943c954bf8afaa8ca0b0db06cd6bd00c0358ba070b198a397d47089e368a8f3dc8446a15e960e4b71b2b12f5b77964c5d8fd49c02f87701830391ed85012a05f2008520c70cfd6b82520894520ae6107ce868e69558ae3424b2cd3369048b2788095cc584c23433c680c001a03794e57db633834aac5311cf0bb7cc9f8c34b9a80485b225eb61abc98869e001a06f134e07cbe905ca81f4e8d3f04c565494f796edd02bdec11991d5acc59ff3a202f90534018219f284931405ec851e9bd9af618307a120945edd5f803b831b47715ad3e11a90dd244f0cd0a980b904c4c98075390000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000003e0000001010101000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002600000000000000000000000723b92452ba80acd1bfd31e98693a5110001249e010408000b05020c070f090a0106030e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000f00000000000000000000000000000000000000000000000000000000025d005000000000000000000000000000000000000000000000000000000000025eb3a800000000000000000000000000000000000000000000000000000000025f4e9d0000000000000000000000000000000000000000000000000000000002616fa00000000000000000000000000000000000000000000000000000000002662a9000000000000000000000000000000000000000000000000000000000026dcbb000000000000000000000000000000000000000000000000000000000027409890000000000000000000000000000000000000000000000000000000002740989000000000000000000000000000000000000000000000000000000000274098900000000000000000000000000000000000000000000000000000000027621e400000000000000000000000000000000000000000000000000000000027621e400000000000000000000000000000000000000000000000000000000027621e400000000000000000000000000000000000000000000000000000000027818c00000000000000000000000000000000000000000000000000000000002920c5a0000000000000000000000000000000000000000000000000000000002920c5a00000000000000000000000000000000000000000000000000000000000000064d9ad85acc4d85b8edd0f07e4910b18c7f60798ea51a9f56deceebd2e3e5e50c6777638458fdcb09a990994bf4842e379bda7d460ebc813f042a23a74956bee22195759fbf4ab55c15d1fa9aacdd6e7775697b49c3a1375639216be095f0d17dabb4937871eea45cc53b22e383efae526f363b6408fe54214b7a7d5d7cd83426f2e73d0fdf8c24f9340e5166ac6f16d80f6aae43a8b7dbc578730e64816f5cc45ead065e26dbca6fdf3e7d564bc13123d0d8e9b8ec72ec0ac85a8633aec867c7000000000000000000000000000000000000000000000000000000000000000651904651ac1c8769ea7e9e143f28c4a57a6ac3b2098cceee5e180cd28b242bb15c379383a79cabfc7b7ac020cab51e07cfbeabdc9b08608aef4edb8c143f28406f728717c324bc6fdbc6f0bc5691169124a62d2c4f4a5c5398298406f5329a7110a4b7d3bd027ce822c3410c896d99a8352f0a816f81e22dd0ae4ddbf4370d6d5fd0adc258df3db664ac3db802aba7665b6d1562c751ca5e0bdd096a7ee2a73f538c88e9d9cc5432b62b32ffa90778e1f66aafa96b220f30aa960de47c2ed19fc001a05c99f4b3ee9e8db9c1f07230d06246dc129151cc7812113992563d5b34908c90a0040d0cefaf2a1eb400914c59e97c7b5adb93ee1225d92b24a51a1e0b2ce508c502f8b4018337e8aa8477359400851e80355e008303291894a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880b844a9059cbb000000000000000000000000f841a830cd94f6f00be674c81f57d5fcbbee2857000000000000000000000000000000000000000000000000000000038869ffb0c001a096cac1bcd991d9503a57399a58bee1194f4a3a6a0d19b153de41e6fc9596757fa04e0675dc544bec595be34d0e39c8d263648e8e17d09b6d78824bef18b536e5e902f8b4018317930e8477359400852fbaf3c2008303d09094a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880b844a9059cbb0000000000000000000000008954b57277a9d7260bb5535afa83d53bf343637c000000000000000000000000000000000000000000000000000000001e742c50c001a0c8702617b1a770e5794633b3a5f6dd33a73e0f7d8a6a5d0b896f2730cc434ba0a0322e4d1c9023b44018a62b636fc1c8161f21624ab38fda44ba940417e46d323602f8b4018334dce88477359400851e80355e008303291894dac17f958d2ee523a2206206994597c13d831ec780b844a9059cbb0000000000000000000000004bb8adce5e7297f2d8c5a2302a68d65eb44158cd000000000000000000000000000000000000000000000000000000000d41fae9c080a041f221a5760e73d3fc8da88f7fe403bf77a6c73e3ed99f97b8cc6d987778ba9aa01bb10c3860a66bb15056d1f8a09ac99273cfde235cb70473b905d0491f26c7be02f8b4018317930f8477359400852fbaf3c2008303d0909488df592f8eb5d7bd38bfef7deb0fbc02cf3778a080b844a9059cbb0000000000000000000000004b7575ef97285f846c944eee2e155bd3ceb65343000000000000000000000000000000000000000000000025e320a2817417f400c080a00bf596f61796e79c557e0d22c1759598ac1dd087d17b897d8a78aaa35ac05b7ea04b9fa664b59577ecc288f1bb10ce093d8085e1bce1648272ec8845155ad588cb02f8d1010c847735940085202170e40083013f3e94084b1c3c81545d370f3634392de611caabff814880b864c47f00270000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000b68696c676572742e657468000000000000000000000000000000000000000000c001a04aced752908560cc21797496850e75ce2a1d106cb8253b40521f7447879d3d75a03e792044fec76604f516c3ee955af79c7b24a6c9639760ad81110329b6c2c0d102f87201018477359400852ad741300082520894a090e606e30bd747d4e6245a1517ebe430f0057e878791c90b4cd41280c080a0a94c2c0391828e9b9b807fa9c1259cdb8b40ce5e223370271e9a59c9db6120f4a05bfe7aa8a8cdac5d906857a5504ea4ac8e67effb04302fb2957067d9bdd84723f90216f90213a09f9076aeb7438dc9e3927bbcff88b1980381d8a5591a5e2323759355dd9ef0a8a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d4934794ea674fdde714fd979de3edf0f56aa9716b898ec8a0343afe56216c786a7da762b125afbab17f7087d4d91973c8882a14839faf7fd4a01dafcd8f132425d9193c8acf6f62276135cc97e6aff9018590ce10711d66684aa0f169809ffad04f682ea4ac33d7a4287609f133b0767ad873dafdfb755657f7d2b901007f6ef7b9b1b7ff57b7dd24dbfd5ddffe1c4597947b37bbfccf65a17f3df97f9bfe3cbfffdb6ff1503419ffdaea7fc5941fbaf92738affb07ca7f7fd1ffef6f29e5d2e1edff7dabfffbaf7f0f7d29e6e046f7fe056f586ff15b74f7a0e68e2ff1ff7b175db73f96f6e7d7ff88fb3e69fbb3fe3ef8febcefecf6f7deb313ca71f2c1fcefcbcbdf7bf056ee7ddb35be27df7e8f4dad7f703d9b2ffbf87f7cbcbd6d5f8f8befffbefe3aeff5f9f0fbdbffbc7bcfdbd4e3bfab1fe7bffffe53eedd785b3ff6cfec5b6df73d93f9f81a8fd66e597432f73eefbf9b59ebe936ff7a24238efaabdfef25afa7fdffbbe5bdf75badfc72efe1f97dc57e7fe9dfff5f5bdfa7873281e8bc688acd83e147ec8401c9c3808401c5a38f84627d9ae08a75732d77657374312d35a01598b74d7f90530f02c9035719061bfec794df6f5a4183aa95ba940c521472168845fe0e67ba2cd6b18517ba6d35fc";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut suite = Suite {
        name: "portal-interop".to_string(),
        description:
            "The portal interop test suite runs a set of scenarios to test interoperability between
        portal network clients"
                .to_string(),
        tests: vec![],
    };

    suite.add(TestSpec {
        name: "OFFER/ACCEPT interop".to_string(),
        description: "".to_string(),
        always_run: false,
        run: test_offer_accept,
        client: None,
    });

    let sim = Simulation::new();
    run_suite(sim, suite).await;
}

async fn run_suite(host: Simulation, suite: Suite) {
    let name = suite.clone().name;
    let description = suite.clone().description;

    let suite_id = host.start_suite(name, description, "".to_string()).await;

    for test in &suite.tests {
        test.run_test(host.clone(), suite_id, suite.clone()).await;
    }

    host.end_suite(suite_id).await;
}

dyn_async! {
   async fn test_offer_accept<'a> (test: &'a mut Test, _client: Option<Client>) {
        // Get all available portal clients
        let clients = test.sim.client_types().await;

        // Test single type of client
        for client in &clients {
                test.run(TwoClientTestSpec {
                    name: format!("OFFER Block Header {} --> {}", client.name, client.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header,
                    client_a: &client.clone(),
                    client_b: &client.clone(),
                }).await;

                test.run(TwoClientTestSpec {
                    name: format!("OFFER Block Body {} --> {}", client.name, client.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_body,
                    client_a: &client.clone(),
                    client_b: &client.clone(),
                }).await;
        }

        // Iterate over all combinations of clients and run the tests
        for combination in clients.iter().combinations(2) {
            let client_a = &combination[0];
            let client_b = &combination[1];

            // Test block header with proof
            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Header {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header,
                    client_a: &(*client_a).clone(),
                    client_b: &(*client_b).clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Header {} --> {}", client_b.name, client_a.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header,
                    client_a: &(*client_b).clone(),
                    client_b: &(*client_a).clone(),
                }
            ).await;

            // Test block body
            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Body {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_body,
                    client_a: &(*client_a).clone(),
                    client_b: &(*client_b).clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Body {} --> {}", client_b.name, client_a.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_body,
                    client_a: &(*client_b).clone(),
                    client_b: &(*client_a).clone(),
                }
            ).await;

            // // Test receipt
            // test.run(
            //     TwoClientTestSpec {
            //         name: format!("OFFER Receipt {} --> {}", client_a.name, client_b.name),
            //         description: "".to_string(),
            //         always_run: false,
            //         run: test_offer_receipt,
            //         client_a: &(*client_a).clone(),
            //         client_b: &(*client_b).clone(),
            //     }
            // ).await;
            //
            // test.run(
            //     TwoClientTestSpec {
            //         name: format!("OFFER Receipt {} --> {}", client_b.name, client_a.name),
            //         description: "".to_string(),
            //         always_run: false,
            //         run: test_offer_receipt,
            //         client_a: &(*client_b).clone(),
            //         client_b: &(*client_a).clone(),
            //     }
            // ).await;
        }
   }
}

dyn_async! {
   async fn test_offer_header<'a> (test: &'a mut Test, client_a: Client, client_b: Client) {
        let header_with_proof_key: HistoryContentKey = serde_json::from_value(json!(HEADER_WITH_PROOF_KEY)).unwrap();
        let header_with_proof_value: HistoryContentItem = serde_json::from_value(json!(HEADER_WITH_PROOF_VALUE)).unwrap();

        let target_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                test.fatal(&format!("Error getting node info: {err:?}"));
                return;
            }
        };

        let _ = client_a.rpc.offer(target_enr, header_with_proof_key.clone(), Some(header_with_proof_value.clone())).await;

        tokio::time::sleep(Duration::from_secs(4)).await;

        let received_content = client_b.rpc.local_content(header_with_proof_key).await;

        match received_content {
            Ok(content) => {
                if content != header_with_proof_value {
                   test.fatal(&format!("Error receiving header with proof: Expected content: {header_with_proof_value:?}, Received content: {content:?}"));
                }
            }
            Err(err) => {
                test.fatal(&format!("Unable to get received content: {err:?}"));
            }
        }
   }
}

dyn_async! {
   async fn test_offer_body<'a> (test: &'a mut Test, client_a: Client, client_b: Client) {
        let block_body_key: HistoryContentKey = serde_json::from_value(json!(BLOCK_BODY_KEY)).unwrap();
        let block_body_value: HistoryContentItem = serde_json::from_value(json!(BLOCK_BODY_VALUE)).unwrap();

        let target_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                test.fatal(&format!("Error getting node info: {err:?}"));
                return;
            }
        };

        let _ = client_a.rpc.offer(target_enr, block_body_key.clone(), Some(block_body_value.clone())).await;

        tokio::time::sleep(Duration::from_secs(4)).await;

        let received_content = client_b.rpc.local_content(block_body_key).await;

        match received_content {
            Ok(content) => {
                if content != block_body_value {
                   test.fatal(&format!("Error receiving block body: Expected content: {block_body_value:?}, Received content: {content:?}"));
                }
            }
            Err(err) => {
                test.fatal(&format!("Unable to get received content: {err:?}"));
            }
        }
   }
}
//
// dyn_async! {
//    async fn test_offer_receipt<'a> (_test: &'a mut Test, client_a: Client, client_b: Client) {
//         println!("Running test_offer_receipt with {:?} and {:?}", client_a.kind, client_b.kind)
//    }
// }