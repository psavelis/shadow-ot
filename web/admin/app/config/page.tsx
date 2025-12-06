'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Settings, Save, RotateCcw, Server, Shield, Users, Coins, 
  Swords, Map, Clock, Globe, Database, Bell, Mail, Lock,
  ChevronDown, ChevronUp, AlertCircle, CheckCircle
} from 'lucide-react'

const configSections = [
  {
    id: 'server',
    name: 'Server Settings',
    icon: Server,
    description: 'Core server configuration',
    settings: [
      { key: 'server.name', label: 'Server Name', type: 'text', value: 'Shadow OT', description: 'Display name for the server' },
      { key: 'server.motd', label: 'Message of the Day', type: 'textarea', value: 'Welcome to Shadow OT!', description: 'Message shown on login' },
      { key: 'server.max_players', label: 'Max Players', type: 'number', value: 1000, description: 'Maximum concurrent players' },
      { key: 'server.login_port', label: 'Login Port', type: 'number', value: 7171, description: 'Login server port' },
      { key: 'server.game_port', label: 'Game Port', type: 'number', value: 7172, description: 'Game server port' },
      { key: 'server.status_port', label: 'Status Port', type: 'number', value: 7173, description: 'Status protocol port' },
    ]
  },
  {
    id: 'rates',
    name: 'Experience & Rates',
    icon: Coins,
    description: 'XP and skill multipliers',
    settings: [
      { key: 'rates.exp', label: 'Experience Rate', type: 'number', value: 10, description: 'Experience multiplier (1x = vanilla)' },
      { key: 'rates.skill', label: 'Skill Rate', type: 'number', value: 5, description: 'Skill training multiplier' },
      { key: 'rates.magic', label: 'Magic Rate', type: 'number', value: 3, description: 'Magic level multiplier' },
      { key: 'rates.loot', label: 'Loot Rate', type: 'number', value: 2, description: 'Loot drop multiplier' },
      { key: 'rates.spawn', label: 'Spawn Rate', type: 'number', value: 1, description: 'Monster respawn multiplier' },
    ]
  },
  {
    id: 'pvp',
    name: 'PvP Settings',
    icon: Swords,
    description: 'Player vs Player rules',
    settings: [
      { key: 'pvp.enabled', label: 'PvP Enabled', type: 'toggle', value: true, description: 'Allow player combat' },
      { key: 'pvp.protection_level', label: 'Protection Level', type: 'number', value: 50, description: 'Level until PvP protection ends' },
      { key: 'pvp.skull_time', label: 'Skull Duration (min)', type: 'number', value: 15, description: 'Minutes skull persists' },
      { key: 'pvp.frag_time', label: 'Frag Time (hours)', type: 'number', value: 24, description: 'Hours until frag expires' },
      { key: 'pvp.daily_frag_limit', label: 'Daily Frag Limit', type: 'number', value: 3, description: 'Frags before red skull' },
      { key: 'pvp.bless_reduction', label: 'Blessing Reduction (%)', type: 'number', value: 30, description: 'Death penalty reduction with blessings' },
    ]
  },
  {
    id: 'security',
    name: 'Security',
    icon: Shield,
    description: 'Account and access security',
    settings: [
      { key: 'security.password_min_length', label: 'Min Password Length', type: 'number', value: 8, description: 'Minimum password characters' },
      { key: 'security.2fa_enabled', label: 'Two-Factor Auth', type: 'toggle', value: true, description: 'Enable 2FA for accounts' },
      { key: 'security.max_login_attempts', label: 'Max Login Attempts', type: 'number', value: 5, description: 'Attempts before lockout' },
      { key: 'security.lockout_duration', label: 'Lockout Duration (min)', type: 'number', value: 30, description: 'Account lockout time' },
      { key: 'security.ip_ban_threshold', label: 'IP Ban Threshold', type: 'number', value: 10, description: 'Failed attempts for IP ban' },
    ]
  },
  {
    id: 'world',
    name: 'World Settings',
    icon: Map,
    description: 'Game world configuration',
    settings: [
      { key: 'world.day_length', label: 'Day Length (min)', type: 'number', value: 60, description: 'Real minutes per game day' },
      { key: 'world.weather_enabled', label: 'Dynamic Weather', type: 'toggle', value: true, description: 'Enable weather system' },
      { key: 'world.house_rent_period', label: 'Rent Period (days)', type: 'number', value: 30, description: 'Days between rent payments' },
      { key: 'world.house_rent_multiplier', label: 'Rent Multiplier', type: 'number', value: 1, description: 'Base rent cost multiplier' },
    ]
  },
  {
    id: 'notifications',
    name: 'Notifications',
    icon: Bell,
    description: 'Alert and notification settings',
    settings: [
      { key: 'notify.discord_webhook', label: 'Discord Webhook', type: 'text', value: '', description: 'Discord webhook URL for alerts' },
      { key: 'notify.email_alerts', label: 'Email Alerts', type: 'toggle', value: true, description: 'Send email notifications' },
      { key: 'notify.alert_on_crash', label: 'Crash Alerts', type: 'toggle', value: true, description: 'Alert on server crash' },
      { key: 'notify.alert_on_high_load', label: 'High Load Alerts', type: 'toggle', value: true, description: 'Alert when load exceeds threshold' },
      { key: 'notify.load_threshold', label: 'Load Threshold (%)', type: 'number', value: 80, description: 'CPU/Memory threshold for alerts' },
    ]
  },
]

export default function ConfigPage() {
  const [expandedSections, setExpandedSections] = useState<string[]>(['server', 'rates'])
  const [modifiedSettings, setModifiedSettings] = useState<Record<string, any>>({})
  const [saving, setSaving] = useState(false)
  const [saved, setSaved] = useState(false)

  const toggleSection = (sectionId: string) => {
    setExpandedSections(prev => 
      prev.includes(sectionId) 
        ? prev.filter(id => id !== sectionId)
        : [...prev, sectionId]
    )
  }

  const handleSettingChange = (key: string, value: any) => {
    setModifiedSettings(prev => ({ ...prev, [key]: value }))
    setSaved(false)
  }

  const getSettingValue = (setting: { key: string; value: any }) => {
    return modifiedSettings[setting.key] ?? setting.value
  }

  const handleSave = async () => {
    setSaving(true)
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1500))
    setSaving(false)
    setSaved(true)
  }

  const handleReset = () => {
    setModifiedSettings({})
    setSaved(false)
  }

  const hasChanges = Object.keys(modifiedSettings).length > 0

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 p-8">
      <div className="max-w-5xl mx-auto">
        {/* Header */}
        <motion.div 
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex items-center justify-between mb-8"
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 bg-gradient-to-br from-purple-500 to-pink-600 rounded-xl flex items-center justify-center">
              <Settings className="w-6 h-6 text-white" />
            </div>
            <div>
              <h1 className="text-2xl font-bold text-white">Server Configuration</h1>
              <p className="text-slate-400">Manage realm settings and parameters</p>
            </div>
          </div>

          <div className="flex items-center gap-3">
            {saved && (
              <motion.div
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                className="flex items-center gap-2 text-emerald-400 bg-emerald-500/10 px-3 py-2 rounded-lg"
              >
                <CheckCircle className="w-4 h-4" />
                <span className="text-sm">Changes saved</span>
              </motion.div>
            )}
            <button
              onClick={handleReset}
              disabled={!hasChanges}
              className="flex items-center gap-2 px-4 py-2 bg-slate-800 text-slate-300 rounded-lg hover:bg-slate-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <RotateCcw className="w-4 h-4" />
              Reset
            </button>
            <button
              onClick={handleSave}
              disabled={!hasChanges || saving}
              className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-lg hover:from-amber-400 hover:to-orange-500 transition disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {saving ? (
                <>
                  <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                  Saving...
                </>
              ) : (
                <>
                  <Save className="w-4 h-4" />
                  Save Changes
                </>
              )}
            </button>
          </div>
        </motion.div>

        {/* Warning Banner */}
        {hasChanges && (
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-amber-500/10 border border-amber-500/30 rounded-xl p-4 mb-6 flex items-center gap-3"
          >
            <AlertCircle className="w-5 h-5 text-amber-400 flex-shrink-0" />
            <p className="text-amber-200 text-sm">
              You have unsaved changes. Some settings may require a server restart to take effect.
            </p>
          </motion.div>
        )}

        {/* Configuration Sections */}
        <div className="space-y-4">
          {configSections.map((section, idx) => (
            <motion.div
              key={section.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: idx * 0.05 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
            >
              {/* Section Header */}
              <button
                onClick={() => toggleSection(section.id)}
                className="w-full flex items-center justify-between p-5 hover:bg-slate-700/30 transition"
              >
                <div className="flex items-center gap-4">
                  <div className="w-10 h-10 bg-slate-700/50 rounded-lg flex items-center justify-center">
                    <section.icon className="w-5 h-5 text-amber-400" />
                  </div>
                  <div className="text-left">
                    <h3 className="text-white font-medium">{section.name}</h3>
                    <p className="text-slate-400 text-sm">{section.description}</p>
                  </div>
                </div>
                {expandedSections.includes(section.id) ? (
                  <ChevronUp className="w-5 h-5 text-slate-400" />
                ) : (
                  <ChevronDown className="w-5 h-5 text-slate-400" />
                )}
              </button>

              {/* Section Content */}
              {expandedSections.includes(section.id) && (
                <motion.div
                  initial={{ opacity: 0, height: 0 }}
                  animate={{ opacity: 1, height: 'auto' }}
                  className="border-t border-slate-700/50"
                >
                  <div className="p-5 space-y-4">
                    {section.settings.map(setting => (
                      <div key={setting.key} className="flex items-start justify-between gap-8">
                        <div className="flex-1">
                          <label className="text-white text-sm font-medium">{setting.label}</label>
                          <p className="text-slate-500 text-xs mt-0.5">{setting.description}</p>
                        </div>
                        <div className="w-64">
                          {setting.type === 'text' && (
                            <input
                              type="text"
                              value={getSettingValue(setting)}
                              onChange={e => handleSettingChange(setting.key, e.target.value)}
                              className="w-full bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:border-amber-500/50"
                            />
                          )}
                          {setting.type === 'number' && (
                            <input
                              type="number"
                              value={getSettingValue(setting)}
                              onChange={e => handleSettingChange(setting.key, parseInt(e.target.value) || 0)}
                              className="w-full bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:border-amber-500/50"
                            />
                          )}
                          {setting.type === 'textarea' && (
                            <textarea
                              value={getSettingValue(setting)}
                              onChange={e => handleSettingChange(setting.key, e.target.value)}
                              rows={2}
                              className="w-full bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:border-amber-500/50 resize-none"
                            />
                          )}
                          {setting.type === 'toggle' && (
                            <button
                              onClick={() => handleSettingChange(setting.key, !getSettingValue(setting))}
                              className={`relative w-12 h-6 rounded-full transition ${
                                getSettingValue(setting) ? 'bg-emerald-500' : 'bg-slate-700'
                              }`}
                            >
                              <div className={`absolute top-1 w-4 h-4 bg-white rounded-full transition-transform ${
                                getSettingValue(setting) ? 'translate-x-7' : 'translate-x-1'
                              }`} />
                            </button>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                </motion.div>
              )}
            </motion.div>
          ))}
        </div>

        {/* Database Actions */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4 }}
          className="mt-8 bg-red-500/5 border border-red-500/20 rounded-xl p-6"
        >
          <div className="flex items-center gap-3 mb-4">
            <Database className="w-5 h-5 text-red-400" />
            <h3 className="text-white font-medium">Danger Zone</h3>
          </div>
          <p className="text-slate-400 text-sm mb-4">
            These actions can have irreversible effects on your server data. Please proceed with caution.
          </p>
          <div className="flex gap-3">
            <button className="px-4 py-2 bg-slate-800 text-slate-300 rounded-lg hover:bg-slate-700 transition text-sm">
              Export Configuration
            </button>
            <button className="px-4 py-2 bg-slate-800 text-slate-300 rounded-lg hover:bg-slate-700 transition text-sm">
              Import Configuration
            </button>
            <button className="px-4 py-2 bg-red-500/20 text-red-400 rounded-lg hover:bg-red-500/30 transition text-sm">
              Reset to Defaults
            </button>
          </div>
        </motion.div>
      </div>
    </div>
  )
}

