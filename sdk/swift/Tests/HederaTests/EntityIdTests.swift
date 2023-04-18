import XCTest

@testable import Hedera

internal final class EntityIdTests: XCTestCase {
    internal func testChecksumOnMainnet() {
        let expected: [String] = [
            "uvnqa", "dfkxr", "lpifi", "tzfmz", "cjcuq", "ktach", "tcxjy", "bmurp",
            "jwrzg", "sgpgx", "hiafh", "rdxmy", "uuuup", "eqscg", "ompjx", "yimro",
            "iejzf", "sahgw", "bweon", "lsbwe", "diuio", "nerqf", "qvoxw", "armfn",
            "knjne", "ujguv", "efecm", "obbkd", "xwyru", "hsvzl", "zjolv", "jfltm",
            "mwjbd", "wsgiu", "godql", "qkayc", "afyft", "kbvnk", "txsvb", "dtqcs",
            "vkipc", "fgfwt", "ixdek", "stamb", "coxts", "mkvbj", "wgsja", "gcpqr",
            "pymyi", "zukfz", "rlcsj", "tqaaa", "xgxhr", "hcupi", "qyrwz", "aupeq",
            "kqmmh", "umjty", "eihbp", "oeejg", "fuwvq", "pqudh", "thrky", "ddosp",
            "mzmag", "wvjhx", "grgpo", "qndxf", "ajbew", "keymn", "bvqyx", "lrogo",
            "pilof", "zeivw", "jagdn", "swdle", "csasv", "mnyam", "wjvid", "gfspu",
            "xwlce", "hsijv", "ljfrm", "vfczd", "fbagu", "owxol", "ysuwc", "iosdt",
            "skplk", "cgmtb", "txffl", "dtcnc", "hjzut", "rfxck", "bbukb", "kxrrs",
            "utozj", "epmha", "oljor", "yhgwi", "hhghj", "prdpa", "ybawr", "gkyei",
            "ouvlz", "xestq", "foqbh", "nyniy", "wikqp", "eshyg", "euakq", "ndxsh",
            "vnuzy", "dxshp", "mhppg", "urmwx", "dbkeo", "llhmf", "tvetw", "cfcbn",
            "wbunx", "elrvo", "mvpdf", "vfmkw", "dpjsn", "lzhae", "ujehv", "ctbpm",
            "lcyxd", "tmweu", "toore", "bylyv", "kijgm", "ssgod", "bcdvu", "jmbdl",
            "rvylc", "afvst", "iptak", "qzqib", "rbiul", "zlgcc", "hvdjt", "qfark",
            "yoxzb", "gyvgs", "pisoj", "xspwa", "gcndr", "omkli", "oocxs", "wyafj",
            "fhxna", "nruur", "wbsci", "elpjz", "mvmrq", "vfjzh", "dphgy", "lzeop",
            "maxaz", "ukuiq", "curqh", "leoxy", "tomfp", "byjng", "kigux", "sseco",
            "bcbkf", "jlyrw", "jnreg", "rxolx", "ahlto", "irjbf", "rbgiw", "zldqn",
            "hvaye", "qeyfv", "yovnm", "gysvd", "halhn", "pkipe", "xufwv", "gedem",
            "ooamd", "wxxtu", "fhvbl", "nrsjc", "wbpqt", "elmyk", "enfku", "mxcsl",
            "vhaac", "dqxht", "maupk", "ukrxb", "cupes", "lemmj", "tojua", "byhbr",
            "klges", "svdmj", "bfaua", "joybr", "ryvji", "aisqz", "ispyq", "rcngh",
            "zmkny", "rthvp", "hyahz", "qhxpq", "yruxh", "hbsey", "plpmp", "xvmug",
            "gfkbx", "ophjo", "wzerf", "pgbyw", "zfulg", "hprsx", "pzpao", "yjmif",
            "gtjpw", "pdgxn", "xnefe", "fxbmv", "ogyum", "gnwcd", "wsoon", "fclwe",
            "nmjdv", "vwglm", "egdtd", "mqbau", "uzyil", "djvqc", "ltsxt", "eaqfk",
            "ufiru", "cpfzl", "kzdhc", "tjaot", "bsxwk", "kcveb", "smsls", "awptj",
            "jgnba", "bnkir", "rscvb", "acacs", "ilxkj", "qvusa", "zfrzr", "hpphi",
        ]

        for (index, expected) in expected.enumerated() {
            let actual = Checksum.generate(for: TopicId(num: UInt64(index)), on: .mainnet).data

            XCTAssertEqual(expected, actual)
        }
    }

    internal func testChecksumOnTestnet() {
        let expected: [String] = [
            "eiyxj", "mswfa", "vctmr", "dmqui", "lwobz", "ugljq", "cqirh", "lafyy",
            "tkdgp", "buaog", "qvlmq", "ariuh", "eigby", "oedjp", "yaarg", "hvxyx",
            "rrvgo", "bnsof", "ljpvw", "vfndn", "mwfpx", "wscxo", "ajaff", "kexmw",
            "uauun", "dwsce", "nspjv", "xomrm", "hkjzd", "rghgu", "iwzte", "ssxav",
            "wjuim", "gfrqd", "qboxu", "zxmfl", "jtjnc", "tpgut", "dleck", "nhbkb",
            "extwl", "otrec", "skolt", "cgltk", "mcjbb", "vygis", "fudqj", "pqaya",
            "zlyfr", "jhvni", "aynzs", "ddlhj", "guipa", "qqfwr", "amdei", "kialz",
            "udxtq", "dzvbh", "nvsiy", "xrpqp", "piicz", "zefkq", "cvcsh", "mqzzy",
            "wmxhp", "giupg", "qerwx", "aapeo", "jwmmf", "tsjtw", "ljcgg", "veznx",
            "yvwvo", "irudf", "snrkw", "cjosn", "mfmae", "wbjhv", "fxgpm", "ptdxd",
            "hjwjn", "rftre", "uwqyv", "esogm", "oolod", "ykivu", "iggdl", "scdlc",
            "byast", "ltyak", "dkqmu", "ngnul", "qxlcc", "atijt", "kpfrk", "ulczb",
            "ehags", "ocxoj", "xyuwa", "husdr", "quros", "zeowj", "homea", "pyjlr",
            "yigti", "gseaz", "pcbiq", "xlyqh", "fvvxy", "oftfp", "ohlrz", "wrizq",
            "fbghh", "nldoy", "vvawp", "eeyeg", "movlx", "uysto", "diqbf", "lsniw",
            "fpfvg", "nzdcx", "wjako", "esxsf", "ncuzw", "vmshn", "dwppe", "mgmwv",
            "uqkem", "dahmd", "dbzyn", "llxge", "tvunv", "cfrvm", "kppdd", "szmku",
            "bjjsl", "jthac", "sdeht", "anbpk", "aoubu", "iyrjl", "riorc", "zslyt",
            "icjgk", "qmgob", "ywdvs", "hgbdj", "ppyla", "xzvsr", "ybofb", "gllms",
            "oviuj", "xfgca", "fpdjr", "nzari", "wixyz", "esvgq", "ncsoh", "vmpvy",
            "voiii", "dyfpz", "micxq", "usafh", "dbxmy", "lluup", "tvscg", "cfpjx",
            "kpmro", "szjzf", "tbclp", "bkztg", "juxax", "seuio", "aorqf", "iyoxw",
            "rimfn", "zsjne", "icguv", "qmecm", "qnwow", "yxtwn", "hhree", "prolv",
            "ybltm", "gljbd", "ovgiu", "xfdql", "fpayc", "nyyft", "oaqsd", "wknzu",
            "eulhl", "neipc", "vofwt", "dydek", "miamb", "urxts", "dbvbj", "llsja",
            "tyrmb", "ciots", "ksmbj", "tcjja", "bmgqr", "jwdyi", "sgbfz", "apynq",
            "izvvh", "bgtcy", "rllpi", "zviwz", "ifgeq", "qpdmh", "yzaty", "hiybp",
            "psvjg", "ycsqx", "gmpyo", "ytngf", "itfsp", "rddag", "znahx", "hwxpo",
            "qguxf", "yqsew", "hapmn", "pkmue", "xukbv", "qbhjm", "gfzvw", "opxdn",
            "wzule", "fjrsv", "ntpam", "wdmid", "enjpu", "mxgxl", "vhefc", "nobmt",
            "dstzd", "mcrgu", "umool", "cwlwc", "lgjdt", "tqglk", "cadtb", "kkbas",
            "styij", "lavqa", "bfock", "jplkb", "rzirs", "ajfzj", "itdha", "rdaor",
        ]

        for (index, expected) in expected.enumerated() {
            let actual = Checksum.generate(for: TopicId(num: UInt64(index)), on: .testnet).data

            XCTAssertEqual(expected, actual)
        }
    }

    internal func testAccountIdChecksum() throws {
        let checksumAddress = try AccountId(num: 1_126_123).toStringWithChecksum(.forMainnet())

        XCTAssertEqual(checksumAddress, "0.0.1126123-ycfte")
    }

    internal func testChecksumOnPreviewnet() {
        let expected: [String] = [
            "nwkes", "wghmj", "eqeua", "nacbr", "vjzji", "dtwqz", "mdtyq", "unrgh",
            "cxony", "lhlvp", "aiwtz", "keubq", "nvrjh", "xroqy", "hnlyp", "rjjgg",
            "bfgnx", "lbdvo", "uxbdf", "esykw", "wjqxg", "gfoex", "jwlmo", "tsiuf",
            "dogbw", "nkdjn", "xgare", "hbxyv", "qxvgm", "atsod", "sklan", "cgiie",
            "fxfpv", "ptcxm", "zpafd", "jkxmu", "tguul", "dcscc", "mypjt", "wumrk",
            "olfdu", "yhcll", "bxztc", "ltxat", "vpuik", "flrqb", "phoxs", "zdmfj",
            "izjna", "svgur", "klzhb", "mqwos", "qhtwj", "adrea", "jzolr", "tvlti",
            "drjaz", "nngiq", "xjdqh", "hfaxy", "yvtki", "irqrz", "minzq", "welhh",
            "gaioy", "pwfwp", "zsdeg", "joalx", "tjxto", "dfvbf", "uwnnp", "eskvg",
            "ijicx", "sffko", "cbcsf", "lwzzw", "vsxhn", "foupe", "pkrwv", "zgpem",
            "qxhqw", "ateyn", "ekcge", "ofznv", "ybwvm", "hxudd", "rtrku", "bposl",
            "llmac", "vhjht", "mybud", "wtzbu", "akwjl", "kgtrc", "ucqyt", "dyogk",
            "nulob", "xqivs", "hmgdj", "ridla", "aicwb", "isads", "rbxlj", "zluta",
            "hvsar", "qfpii", "ypmpz", "gzjxq", "pjhfh", "xtemy", "xuwzi", "geugz",
            "ooroq", "wyowh", "fimdy", "nsjlp", "wcgtg", "emeax", "mwbio", "vfyqf",
            "pcrcp", "xmokg", "fwlrx", "ogizo", "wqghf", "fadow", "nkawn", "vtyee",
            "edvlv", "mnstm", "mplfw", "uzinn", "djfve", "ltdcv", "udakm", "cmxsd",
            "kwuzu", "tgshl", "bqppc", "kamwt", "kcfjd", "smcqu", "avzyl", "jfxgc",
            "rpunt", "zzrvk", "ijpdb", "qtmks", "zdjsj", "hnhaa", "hozmk", "pywub",
            "yiubs", "gsrjj", "pcora", "xmlyr", "fwjgi", "oggnz", "wqdvq", "fabdh",
            "fbtpr", "nlqxi", "vvoez", "eflmq", "mpiuh", "uzgby", "djdjp", "ltarg",
            "ucxyx", "cmvgo", "consy", "kylap", "tiiig", "bsfpx", "kccxo", "smaff",
            "avxmw", "jfuun", "rpsce", "zzpjv", "abhwf", "ilfdw", "qvcln", "zezte",
            "hoxav", "pyuim", "yirqd", "gsoxu", "pcmfl", "xmjnc", "xobzm", "fxzhd",
            "ohwou", "wrtwl", "fbrec", "nlolt", "vvltk", "efjbb", "mpgis", "uzdqj",
            "dmctk", "lwabb", "ufxis", "cpuqj", "kzrya", "tjpfr", "btmni", "kdjuz",
            "snhcq", "kuekh", "aywwr", "jiuei", "rsrlz", "acotq", "immbh", "qwjiy",
            "zggqp", "hqdyg", "qabfx", "igyno", "sgqzy", "aqohp", "jalpg", "rkiwx",
            "zugeo", "iedmf", "qoatw", "yxybn", "hhvje", "zosqv", "ptldf", "ydikw",
            "gnfsn", "oxdae", "xhahv", "fqxpm", "oauxd", "wkseu", "eupml", "xbmuc",
            "ngfgm", "vqcod", "dzzvu", "mjxdl", "utulc", "ddrst", "lnpak", "txmib",
            "chjps", "uogxj", "kszjt", "tcwrk", "bmtzb", "jwrgs", "sgooj", "aqlwa",
        ]

        for (index, expected) in expected.enumerated() {
            let actual = Checksum.generate(for: TopicId(num: UInt64(index)), on: .previewnet).data

            XCTAssertEqual(expected, actual)
        }
    }

    internal func testChecksumOnCustomNetwork() {
        let expected: [String] = [
            "kenvf", "solcw", "ayikn", "jifse", "rsczv", "acahm", "ilxpd", "qvuwu",
            "zfsel", "hppmc", "wrakm", "gmxsd", "kduzu", "tzshl", "dvppc", "nrmwt",
            "xnkek", "hjhmb", "rfets", "bbcbj", "srunt", "cnrvk", "gepdb", "qamks",
            "zwjsj", "jshaa", "toehr", "dkbpi", "nfywz", "xbweq", "osora", "yolyr",
            "cfjgi", "mbgnz", "vxdvq", "ftbdh", "poyky", "zkvsp", "jgtag", "tcqhx",
            "ktiuh", "upgby", "ygdjp", "icarg", "rxxyx", "btvgo", "lpsof", "vlpvw",
            "fhndn", "pdkle", "gucxo", "izaff", "mpxmw", "wluun", "ghsce", "qdpjv",
            "zzmrm", "jvjzd", "trhgu", "dneol", "vdxav", "ezuim", "iqrqd", "smoxu",
            "cimfl", "mejnc", "wagut", "fweck", "psbkb", "znyrs", "rerec", "baolt",
            "erltk", "onjbb", "yjgis", "ifdqj", "sbaya", "bwyfr", "lsvni", "vosuz",
            "nflhj", "xbipa", "asfwr", "kodei", "ukalz", "efxtq", "obvbh", "xxsiy",
            "htpqp", "rpmyg", "jgfkq", "tccsh", "wszzy", "goxhp", "qkupg", "agrwx",
            "kcpeo", "tymmf", "dujtw", "nqhbn", "wqgmo", "faduf", "nkbbw", "vtyjn",
            "edvre", "mnsyv", "uxqgm", "dhnod", "lrkvu", "ubidl", "udapv", "cmxxm",
            "kwvfd", "tgsmu", "bqpul", "kancc", "skkjt", "auhrk", "jeezb", "rocgs",
            "lkutc", "tusat", "cepik", "komqb", "syjxs", "bihfj", "jsena", "scbur",
            "alzci", "ivwjz", "ixowj", "rhmea", "zrjlr", "ibgti", "qleaz", "yvbiq",
            "heyqh", "povxy", "xytfp", "giqng", "gkizq", "oughh", "xedoy", "foawp",
            "nxyeg", "whvlx", "ersto", "nbqbf", "vlniw", "dvkqn", "dxdcx", "mhako",
            "uqxsf", "dauzw", "lkshn", "tuppe", "cemwv", "kokem", "syhmd", "bietu",
            "bjxge", "jtunv", "sdrvm", "anpdd", "ixmku", "rhjsl", "zrhac", "ibeht",
            "qlbpk", "yuyxb", "ywrjl", "hgorc", "pqlyt", "yajgk", "gkgob", "oudvs",
            "xebdj", "fnyla", "nxvsr", "whtai", "wjlms", "etiuj", "ndgca", "vndjr",
            "dxari", "mgxyz", "uqvgq", "dasoh", "lkpvy", "tundp", "twfpz", "cgcxq",
            "kqafh", "szxmy", "bjuup", "jtscg", "sdpjx", "anmro", "ixjzf", "rhhgw",
            "zugjx", "iedro", "qoazf", "yxygw", "hhvon", "prswe", "ybqdv", "glnlm",
            "ovktd", "hciau", "xhane", "fqxuv", "oavcm", "wkskd", "eupru", "nemzl",
            "vokhc", "dyhot", "miewk", "epceb", "oouql", "wyryc", "fipft", "nsmnk",
            "wcjvb", "emhcs", "mwekj", "vgbsa", "dpyzr", "vwwhi", "mbots", "ulmbj",
            "cvjja", "lfgqr", "tpdyi", "bzbfz", "kiynq", "ssvvh", "bctcy", "tjqkp",
            "joiwz", "rygeq", "aidmh", "isaty", "rbybp", "zlvjg", "hvsqx", "qfpyo",
            "ypngf", "qwknw", "hbdag", "plahx", "xuxpo", "geuxf", "oosew", "wypmn",
        ]

        for (index, expected) in expected.enumerated() {
            let actual = Checksum.generate(for: TopicId(num: UInt64(index)), on: LedgerId(Data([12, 31, 0, 2]))).data

            XCTAssertEqual(expected, actual)
        }
    }
}
