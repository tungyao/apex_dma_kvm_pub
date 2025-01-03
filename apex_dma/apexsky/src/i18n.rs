// use std::collections::HashMap;

use fluent::{FluentBundle, FluentResource};

use strum::{EnumString, EnumVariantNames};
use strum_macros::EnumIter;
use sys_locale::get_locale;
// Used to provide a locale for the bundle.
use unic_langid::LanguageIdentifier;

#[derive(Debug, EnumString, EnumVariantNames, strum::Display, EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum MessageId {
    HelloWorld,
    MenuValueEnabled,
    MenuValueDisabled,
    MainMenuTitle,
    TitleMainSetting,
    TitleLootSetting,
    TitlePlayerSetting,
    TitleAimSetting,
    TitleExtraSkill,
    TitleOtherSetting,
    MenuItemFiringRange,
    MenuItemKeyboard,
    MenuItemGamepad,
    MenuItemItemGlow,
    MenuItemPlayerGlow,
    MenuItemSmoothValue,
    MenuItemSmoothSubValue,
    InputPromptSmoothValue,
    InputPromptSmoothSubValue,
    InfoInvalidSmoothValue,
    MenuItemChangeBoneAim,
    MenuValueBoneAuto,
    MenuValueBoneNearest,
    MenuValueBoneHead,
    MenuValueBoneNeck,
    MenuValueBoneChest,
    MenuValueBoneGutShut,
    MenuValueBoneUnknown,
    InputPromptBoneValue,
    InfoInvalidBoneValue,
    InfoInvalidValue,
    MenuItemLootGlowFilled,
    MenuItemPlayerGlowFilled,
    MenuItemPlayerOutlineSize,
    InputPromptPlayerOutlines,
    InputPromptPlayerDistance,
    InfoPlayerOutlineUpdated,
    InfoInvalidOutlineSize,
    InfoInvalidDistance,
    MenuItemUpdateGlowColors,
    MenuItemChangeAdsFov,
    InputPromptAdsFov,
    InfoInvalidAdsFov,
    MenuItemChangeNonAdsFov,
    InputPromptNonAdsFov,
    InfoInvalidNonAdsFov,
    MenuItemChangeFlickFov,
    InputPromptFlickFov,
    InfoInvalidFlickFov,
    MenuItemChangeRecoilPitch,
    InputPromptRecoilPitch,
    MenuItemChangeRecoilYaw,
    InputPromptRecoilYaw,
    InfoInvalidRecoil,
    MenuItemFlickNearest,
    MenuItemSuperGlide,
    MenuItemItemFilterSettings,
    MenuItemHotkeySettings,
    MenuItemDeathBoxes,
    MenuItemSaveSettings,
    MenuItemLoadSettings,
    InfoSaved,
    InfoFailed,
    InfoLoaded,
    InfoFallbackConfig,
    MenuItemToggleNadeAim,
    MenuValueNoNadeAim,
    MenuValueNadeAimOn,
    MenuItemToggleOnevone,
    MenuItemToggleNoRecoil,
    MenuItemSetFpsPredict,
    MenuItemWeaponPredict,
    MenuValueCalcFps,
    InputPromptFpsPredict,
    MenuItemPlayerArmorGlowColor,
    MenuItemWeaponModelGlow,
    InfoWeaponModelGlow,
    MenuItemToggleOverlay,
    MenuValueNoOverlay,
    MenuValueExternalOverlay,
    InfoExpectingValueCount,
    InfoCannotParseInputValues,
    InfoValuesOutOfRange,
    InputPromptColorRgb,
    GlowColorMenuTitle,
    MenuItemGlowColors,
    ColorItemNotVizTarget,
    ColorItemVizTarget,
    ColorItemKnockedTarget,
    InfoGlowColorsUpdated,
    MenuItemBackToMainMenu,
    InputPromptKeycode,
    HotkeyMenuTitle,
    HotkeyItemAimbot1,
    HotkeyItemAimbot2,
    HotkeyItemTriggerBot,
    HotkeyItemFlickBot,
    HotkeyItemQuickGlow,
    HotkeyItemQuickAim,
    HotkeyItemAlgsRadar,
    MenuItemKeyCodes,
    ItemFilterMenuTitle,
    InfoInvalidKeycode,
    ItemLightWeapons,
    ItemHeavyWeapons,
    ItemEnergyWeapons,
    ItemSniperWeapons,
    ItemArmors,
    ItemHealing,
    ItemNades,
    ItemBackpacks,
    ItemHopUps,
    ItemScopes,
    RedIsDisable,
    GreedIsEnabled,
    LightWeaponsMenuTitle,
    LightWeaponsSection,
    WeaponP2020,
    WeaponRe45,
    WeaponAlternator,
    WeaponR99,
    WeaponR301,
    WeaponM600,
    WeaponG7Scout,
    LootLightAmmo,
    LightWeaponMagsSection,
    LootLightWeaponMag,
    WeaponStocksSection,
    LootStandardStock,
    LootSniperStock,
    WeaponSuppressorsSection,
    LootWeaponSuppressors,
    WeaponLasersSection,
    LootWeaponLasers,
    LootQuickdrawHolster,
    WeaponHopUpsMenuTitle,
    WeaponHopUpsSection,
    LootTurboCharger,
    LootSkullPiecer,
    LootHammerPoints,
    LootDisruptorRounds,
    LootBoostedLoader,
    LootAnvilReceiver,
    LootDoubletapTrigger,
    LootDualShell,
    LootKineticFeeder,
    HeavyWeaponsMenuTitle,
    HeavyWeaponsSection,
    WeaponFlatline,
    WeaponHemlock,
    Weapon3030Repeater,
    WeaponRampage,
    WeaponProwler,
    WeaponCarSmg,
    LootHeavyAmmo,
    HeavyWeaponMagsSection,
    LootHeavyWeaponMag,
    EnergyWeaponsMenuTitle,
    EnergyWeaponsSection,
    WeaponLStar,
    WeaponNemesis,
    WeaponHavoc,
    WeaponDeovtion,
    WeaponTripleTake,
    WeaponVolt,
    LootEnergyAmmo,
    EnergyWeaponMagsSection,
    LootEnergyWeaponMag,
    SniperWeaponsMenuTitle,
    SniperWeaponsSection,
    WeaponWingman,
    WeaponLongbow,
    WeaponKraber,
    WeaponChargeRifle,
    WeaponSentinel,
    WeaponBow,
    LootSniperAmmo,
    SniperWeaponMagsSection,
    LootSniperWeaponMag,
    ArmorsMenuTitle,
    ArmorsSection,
    LootBodyShield,
    LootEvoShield,
    HelmetsSection,
    LootHelmet,
    KnockdownShieldsSection,
    LootKnockdownShield,
    HealingItemsMenuTitle,
    HealingItemsSection,
    LootAccelerant,
    LootPhoenix,
    LootSmallHealth,
    LootLargeHealth,
    LootSmallShieldBatt,
    LootLargeShieldBatt,
    LootMobileRespawn,
    NadesMenuTitle,
    NadeItemsSection,
    LootFragGrenade,
    LootArcStar,
    LootThermite,
    BackpacksMenuTitle,
    BackpacksSection,
    LootLightBackpack,
    LootMediumBackpack,
    LootHeavyBackpack,
    LootGoldBackpack,
    ScopesMenuTitle,
    ScopesSection,
    Loot1xHcog,
    Loot2xHcog,
    Loot1xHolo,
    Loot1x2xHolo,
    LootOpticThreat,
    Loot3xHcog,
    Loot2x4xAog,
    Loot6xSniperOptic,
    Loot4x8xSniperOptic,
    LootSniperThreat,
    KeyCodesMenuTitle,
    Keycode108Mouse1Left,
    Keycode109Mouse2Right,
    Keycode110Mouse3Middle,
    Keycode111Mouse4Side,
    Keycode112Mouse5Side,
    MenuItemBackToHotkeyMenu,
    MenuValuePrefix,
    MenuValueSuffix,
    LootLevel1Name,
    LootLevel2Name,
    LootLevel3Name,
    LootLevel4Name,
    LootLevel5Name,
    MenuItemFavoritePlayerGlow,
    // MenuItemKbdBacklightCtrl,
    MenuItemBowChargeRifleAim,
    MenuItemTriggerAutoShot,
    MenuItemSuperGrpple,
    MenuItemAutoTapstrafe,
    MenuItemPlayerGlowDist,
    WeaponsPredictMenuTitle,
    InputPromptWeaponsPredict,
    InfoInvalidPredict,
}

pub fn get_fluent_bundle() -> FluentBundle<FluentResource> {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    get_bundle(&locale)
}

fn get_bundle<'a>(accept_locale: &'a str) -> FluentBundle<FluentResource> {
    let (locale, ftl_string) = match accept_locale {
        "zh-TW" => (
            "zh-TW",
            include_str!("../resource/i18n/zh-TW.ftl").to_owned(),
        ),
        "zh-CN" => (
            "zh-CN",
            include_str!("../resource/i18n/zh-CN.ftl").to_owned(),
        ),
        _ => (
            "en-US",
            include_str!("../resource/i18n/en-US.ftl").to_owned(),
        ),
    };
    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

    let lang_id: LanguageIdentifier = locale.parse().expect("Parsing failed.");
    let mut bundle = FluentBundle::new(vec![lang_id]);

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");
    bundle
}

#[macro_export]
macro_rules! i18n_msg {
    ( $bundle:expr, $message_id:ident) => {{
        use crate::i18n::MessageId;
        let msg = $bundle
            .get_message(&MessageId::$message_id.to_string())
            .expect("Message doesn't exist.");
        let mut errors = vec![];
        let pattern = msg.value().expect("Message has no value.");
        let value = $bundle.format_pattern(&pattern, None, &mut errors);
        value
    }};
}

#[macro_export]
macro_rules! i18n_msg_format {
    ( $bundle:expr, $message_id:ident, $args:expr) => {{
        use crate::i18n::MessageId;
        let msg = $bundle
            .get_message(&MessageId::$message_id.to_string())
            .expect("Message doesn't exist.");
        let mut errors = vec![];
        let pattern = msg.value().expect("Message has no value.");
        let value = $bundle.format_pattern(&pattern, Some(&$args), &mut errors);
        value
    }};
}
/*
pub fn get<'a, 'b>(
    accept_language: &'a str,
    message_ids: Vec<&'static str>,
) -> Result<HashMap<&'static str, String>, ()> {
    let bundle = get_bundle(accept_language);
    let mut result = HashMap::with_capacity(message_ids.len());
    let mut errors = vec![];
    for message_id in message_ids {
        let msg = bundle
            .get_message(message_id)
            .expect("Message doesn't exist.");
        let pattern = msg.value().expect("Message has no value.");
        let value = bundle.format_pattern(&pattern, None, &mut errors);
        errors.clear();
        result.insert(message_id, value.to_string());
    }
    Ok(result)
}
*/
#[test]
fn get_all_message_ids() {
    use strum::IntoEnumIterator;
    println!("- i18n message id -----------");
    MessageId::iter().for_each(|i| println!("{} = ", i));
    println!("- i18n message id END--------");
}
