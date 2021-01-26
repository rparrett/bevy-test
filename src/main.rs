use bevy::{log::LogSettings, prelude::*};

#[derive(Default)]
struct State {
    index: usize,
}

static FONT_SIZE: f32 = 48.;

static TEXT: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()あいうえおかきくけこがぎぐげごさしすせそざじずぜぞたちつてとだぢづでどなにぬねのはひふへほばびぶべぼぱぴぷぺぽまみむめもやゆよらりるれろわゐゑをんアイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヰヱヲン娃阿挨逢葵茜渥旭葦芦梓斡宛絢綾鮎或粟庵按闇鞍杏伊夷惟椅畏謂亥郁磯溢茨鰯允胤蔭烏迂卯鵜窺丑碓臼唄姥厩瓜閏噂云叡曳瑛榎堰奄燕艶苑薗於甥旺襖岡荻臆桶牡俺伽嘉珂禾茄蝦嘩迦霞俄峨牙臥駕廻恢魁晦芥蟹凱崖蓋鎧浬馨柿笠樫梶恰葛叶椛樺鞄兜蒲釜鎌鴨茅萱粥瓦侃柑竿莞韓巌玩雁伎嬉毅畿稀徽亀祇誼掬鞠桔橘砧杵汲灸笈鋸亨匡卿喬蕎饗尭桐僅巾錦欣欽禽芹衿玖矩駈駒喰寓串櫛釧屑窟沓窪熊隈栗鍬袈祁圭慧桂稽詣戟隙桁訣倦喧拳捲牽硯鍵絃舷諺乎糊袴胡虎跨伍吾梧檎瑚醐鯉倖勾宏巷庚弘昂晃杭梗浩紘腔膏閤鴻劫壕轟忽惚此頃昏些叉嵯沙瑳裟坐哉塞采犀砦冴阪堺榊肴埼鷺朔柵窄笹拶薩皐錆晒撒燦珊纂讃仔孜斯獅爾而蒔汐鹿竺雫悉篠偲柴縞紗灼錫惹洲蒐蹴輯峻竣舜駿楯淳醇曙渚恕哨嘗庄捷昌梢樟湘菖蕉裳鞘丞杖穣埴拭燭晋榛秦芯壬腎訊諏須厨逗翠錐瑞嵩雛菅頗雀裾摺凄棲栖醒戚蹟碩尖撰煎穿羨詮閃膳噌曾曽楚疏蘇遡叢爽宋惣槍漕綜聡蒼捉袖其揃遜汰舵楕陀堆戴苔黛鯛醍鷹瀧啄托琢茸凧只辰巽竪辿樽誰坦旦歎湛耽檀弛智馳筑註酎猪喋寵帖暢牒蝶椎槌槻佃柘辻蔦綴椿紬爪鶴悌挺梯汀禎諦蹄鄭釘鼎擢填纏貼顛兎堵杜砥套宕嶋燈董藤憧撞瞳萄栃鳶寅酉惇敦沌遁頓奈那凪薙謎灘捺鍋楢馴楠汝匂賑虹廿濡禰祢捻乃之埜巴播杷琶芭盃煤這秤萩柏箔曝莫函箸肇筈幡畠鳩塙隼斑汎挽磐蕃庇斐緋樋枇毘琵眉柊疋彦菱肘畢桧媛紐彪瓢豹廟彬瀕冨斧芙阜撫葡蕪楓葺蕗淵吻焚蔽頁碧瞥篇娩鞭圃甫輔戊菩峯捧朋萌蓬蜂鋒鳳鵬貌卜睦勃殆幌昧哩槙枕柾鱒亦俣沫迄麿蔓巳箕蜜湊蓑稔牟椋冥姪孟蒙儲勿餅尤籾貰也冶耶弥靖佑宥柚湧祐邑輿傭妖楊耀蓉遥淀螺洛嵐藍蘭李梨璃裡掠劉溜琉龍侶亮凌梁瞭稜諒遼淋琳鱗麟瑠伶嶺怜玲憐漣煉簾蓮呂魯櫓狼麓禄肋倭脇鷲亙亘詫藁蕨椀碗乘亞佛侑來俐傳僞價儉兒凉凛凰刹剩劍勁勳卷單嚴圈國圓團壞壘壯壽奎奧奬孃實寢將專峽崚巖已帶廣廳彈彌彗從徠恆惡惠惺愼應懷戰戲拔拜拂搜搖攝收敍昊昴晏晄晝晨晟暉曉曖檜栞條梛椰榮樂樣橙檢櫂櫻盜毬氣洸洵淨滉漱滯澁澪濕煌燒燎燿爭爲狹默獸珈珀琥瑶疊皓盡眞眸碎祕祿禪禮稟稻穗穰笙粹絆綺綸縣縱纖羚翔飜聽脩臟與苺茉莊莉菫萠萬蕾藏藝藥衞裝覽詢諄謠讓賣赳轉迪逞醉釀釉鎭鑄陷險雜靜頌顯颯騷驍驗髮鷄麒黎齊堯槇遙凜熙挨垢憧宛或椅伊炒嘘嬉噂餌於岡俺嘩鍵崖賭籠霞嘗鞄釜噛瓦稀稽蹴喧梢忽此頃沙匙拶爽叱腫繍醤尻芯腎隙凄裾咳咀蘇袖其剃揃汰叩只溜誰旦蛋馳蝶呟壷爪吊頓丼那奈謎撫鍋匂賑睨濡覗喉呪狙這箸筈貼髭膝肘瞳紐藤蓋吠頬殆惚蒔撒枕股眉勿尤貰闇茹嵐呂脇湧僅碗阿葵茜旭梓虻粟庵鞍杏磯鰯允蔭烏迂鵜窺丑臼唄姥瓜閏云堰怨燕艶苑荻伽苛俄牙柿笠恰葛叶樺兜鎌鴨茅粥姦竿韓玩癌雁毅畿亀祇橘僑匡卿桐錦倶狗駒寓串櫛窪熊隈栗桂詣拳捲姑狐虎跨鯉杭紘腔壕濠麹痕些叉塞采砦冴榊笹纂讃斬獅鹿悉柴錫惹讐竣曙渚哨娼昌杖埴拭須瑞雛雀摺棲醒戚羨腺閃膳惣掻槍綜舵楕堆戴黛鯛鷹凧辰巽鱈樽綻酎猪喋暢諜銚掴佃辻蔦綴椿潰鶴挺轍纏澱兎堵妬寅酉沌灘捺楠虹葱撚巴罵牌萩柏曝鳩隼斑汎庇斐媛淵僻";

fn change_text(
    mut query: Query<&mut Text>,
    mut state: ResMut<State>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = TEXT.chars().skip(state.index).take(1).collect();
        
        state.index += 1;
        if state.index > TEXT.chars().count() {
            state.index = 0;
        }
    }
}

fn setup(commands: &mut Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(CameraUiBundle::default());

    commands.spawn(TextBundle {
        text: Text::with_section(
            "".to_string(),
            TextStyle {
                font: asset_server.load("fonts/NotoSansJP-Light.otf"),
                font_size: FONT_SIZE,
                color: Color::GREEN,
            },
            TextAlignment::default(),
        ),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .add_resource(LogSettings {
            level: bevy::log::Level::TRACE,
            filter: "bevy_webgl2=trace".into(),
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<State>()
        .add_startup_system(setup.system())
        .add_system(change_text.system())
        .run();
}
