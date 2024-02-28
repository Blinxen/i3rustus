// This is the netlink header size without the payload
pub const NETLINK_HEADER_SIZE: usize = 16;
// See https://git.kernel.org/pub/scm/linux/kernel/git/linville/wireless.git/tree/include/uapi/linux/nl80211.h?id=HEAD
//
// nl80211_commands
pub const NL80211_CMD_GET_INTERFACE: i32 = 5;
pub const NL80211_CMD_GET_STATION: i32 = 17;
pub const NL80211_CMD_GET_SCAN: i32 = 32;

// nl80211_attrs
pub const NL80211_ATTR_IFINDEX: i32 = 3;
pub const NL80211_ATTR_MAC: i32 = 6;
pub const NL80211_ATTR_STA_INFO: i32 = 21;
pub const NL80211_ATTR_BSS: i32 = 47;
pub const NL80211_ATTR_SSID: i32 = 52;

// nl80211_bss
pub const NL80211_BSS_FREQUENCY: i32 = 2;
pub const NL80211_BSS_INFORMATION_ELEMENTS: i32 = 6;
pub const NL80211_BSS_STATUS: i32 = 9;

// nl80211_bss_status {
pub const NL80211_BSS_STATUS_ASSOCIATED: u32 = 1;
pub const NL80211_BSS_STATUS_IBSS_JOINED: u32 = 2;