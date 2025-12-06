'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  User, 
  Mail, 
  Key, 
  Shield, 
  Bell, 
  Globe,
  Camera,
  Save,
  Eye,
  EyeOff,
  Check
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'
import * as Switch from '@radix-ui/react-switch'

export default function SettingsPage() {
  const [showCurrentPassword, setShowCurrentPassword] = useState(false)
  const [showNewPassword, setShowNewPassword] = useState(false)
  const [saved, setSaved] = useState(false)

  const handleSave = () => {
    setSaved(true)
    setTimeout(() => setSaved(false), 2000)
  }

  return (
    <div className="max-w-4xl space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <h1 className="text-2xl font-display font-bold mb-1">Account Settings</h1>
        <p className="text-shadow-400">Manage your account preferences and security</p>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
      >
        <Tabs.Root defaultValue="profile">
          <Tabs.List className="flex space-x-1 bg-shadow-900/50 rounded-xl p-1 mb-6">
            <Tabs.Trigger
              value="profile"
              className="flex items-center space-x-2 px-4 py-2.5 rounded-lg text-sm font-medium transition-all data-[state=active]:bg-accent-500 data-[state=active]:text-white data-[state=inactive]:text-shadow-400 data-[state=inactive]:hover:text-white"
            >
              <User className="w-4 h-4" />
              <span>Profile</span>
            </Tabs.Trigger>
            <Tabs.Trigger
              value="security"
              className="flex items-center space-x-2 px-4 py-2.5 rounded-lg text-sm font-medium transition-all data-[state=active]:bg-accent-500 data-[state=active]:text-white data-[state=inactive]:text-shadow-400 data-[state=inactive]:hover:text-white"
            >
              <Shield className="w-4 h-4" />
              <span>Security</span>
            </Tabs.Trigger>
            <Tabs.Trigger
              value="notifications"
              className="flex items-center space-x-2 px-4 py-2.5 rounded-lg text-sm font-medium transition-all data-[state=active]:bg-accent-500 data-[state=active]:text-white data-[state=inactive]:text-shadow-400 data-[state=inactive]:hover:text-white"
            >
              <Bell className="w-4 h-4" />
              <span>Notifications</span>
            </Tabs.Trigger>
            <Tabs.Trigger
              value="preferences"
              className="flex items-center space-x-2 px-4 py-2.5 rounded-lg text-sm font-medium transition-all data-[state=active]:bg-accent-500 data-[state=active]:text-white data-[state=inactive]:text-shadow-400 data-[state=inactive]:hover:text-white"
            >
              <Globe className="w-4 h-4" />
              <span>Preferences</span>
            </Tabs.Trigger>
          </Tabs.List>

          {/* Profile Tab */}
          <Tabs.Content value="profile" className="space-y-6">
            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-6">Profile Information</h2>
              
              {/* Avatar */}
              <div className="flex items-center space-x-6 mb-8">
                <div className="relative">
                  <div className="w-24 h-24 bg-gradient-to-br from-accent-500 to-purple-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-3xl font-bold">S</span>
                  </div>
                  <button className="absolute bottom-0 right-0 w-8 h-8 bg-shadow-800 border border-shadow-600 rounded-full flex items-center justify-center hover:bg-shadow-700 transition-colors">
                    <Camera className="w-4 h-4 text-white" />
                  </button>
                </div>
                <div>
                  <h3 className="font-medium text-white mb-1">Profile Picture</h3>
                  <p className="text-shadow-400 text-sm mb-2">JPG, PNG or GIF. Max 2MB.</p>
                  <button className="text-accent-500 hover:text-accent-400 text-sm font-medium">
                    Upload new picture
                  </button>
                </div>
              </div>

              <div className="grid md:grid-cols-2 gap-6">
                <div>
                  <label className="block text-sm font-medium text-shadow-300 mb-2">
                    Account Name
                  </label>
                  <input
                    type="text"
                    defaultValue="ShadowMaster"
                    className="input"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-shadow-300 mb-2">
                    Display Name
                  </label>
                  <input
                    type="text"
                    defaultValue="Shadow Master"
                    className="input"
                  />
                </div>
                <div className="md:col-span-2">
                  <label className="block text-sm font-medium text-shadow-300 mb-2">
                    Email Address
                  </label>
                  <div className="relative">
                    <Mail className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
                    <input
                      type="email"
                      defaultValue="shadowmaster@email.com"
                      className="input pl-12"
                    />
                  </div>
                </div>
                <div className="md:col-span-2">
                  <label className="block text-sm font-medium text-shadow-300 mb-2">
                    Bio
                  </label>
                  <textarea
                    rows={3}
                    placeholder="Tell us about yourself..."
                    className="input resize-none"
                    defaultValue="Veteran player since 2007. Guild leader of Dark Legion."
                  />
                </div>
              </div>

              <div className="flex justify-end mt-6">
                <button onClick={handleSave} className="btn-primary flex items-center space-x-2">
                  {saved ? <Check className="w-5 h-5" /> : <Save className="w-5 h-5" />}
                  <span>{saved ? 'Saved!' : 'Save Changes'}</span>
                </button>
              </div>
            </div>
          </Tabs.Content>

          {/* Security Tab */}
          <Tabs.Content value="security" className="space-y-6">
            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-6">Change Password</h2>
              
              <div className="space-y-4 max-w-md">
                <div>
                  <label className="block text-sm font-medium text-shadow-300 mb-2">
                    Current Password
                  </label>
                  <div className="relative">
                    <Key className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
                    <input
                      type={showCurrentPassword ? 'text' : 'password'}
                      placeholder="Enter current password"
                      className="input pl-12 pr-12"
                    />
                    <button
                      type="button"
                      onClick={() => setShowCurrentPassword(!showCurrentPassword)}
                      className="absolute right-4 top-1/2 -translate-y-1/2 text-shadow-500 hover:text-white"
                    >
                      {showCurrentPassword ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
                    </button>
                  </div>
                </div>
                <div>
                  <label className="block text-sm font-medium text-shadow-300 mb-2">
                    New Password
                  </label>
                  <div className="relative">
                    <Key className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
                    <input
                      type={showNewPassword ? 'text' : 'password'}
                      placeholder="Enter new password"
                      className="input pl-12 pr-12"
                    />
                    <button
                      type="button"
                      onClick={() => setShowNewPassword(!showNewPassword)}
                      className="absolute right-4 top-1/2 -translate-y-1/2 text-shadow-500 hover:text-white"
                    >
                      {showNewPassword ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
                    </button>
                  </div>
                </div>
                <div>
                  <label className="block text-sm font-medium text-shadow-300 mb-2">
                    Confirm New Password
                  </label>
                  <input
                    type="password"
                    placeholder="Confirm new password"
                    className="input"
                  />
                </div>
                <button className="btn-primary">Update Password</button>
              </div>
            </div>

            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-6">Two-Factor Authentication</h2>
              
              <div className="flex items-center justify-between p-4 bg-shadow-800/50 rounded-lg">
                <div>
                  <h3 className="font-medium text-white mb-1">Authenticator App</h3>
                  <p className="text-shadow-400 text-sm">Use an authenticator app to get verification codes</p>
                </div>
                <Switch.Root className="w-11 h-6 bg-shadow-700 rounded-full relative data-[state=checked]:bg-accent-500 transition-colors">
                  <Switch.Thumb className="block w-5 h-5 bg-white rounded-full shadow-lg transition-transform translate-x-0.5 data-[state=checked]:translate-x-5" />
                </Switch.Root>
              </div>

              <div className="flex items-center justify-between p-4 bg-shadow-800/50 rounded-lg mt-4">
                <div>
                  <h3 className="font-medium text-white mb-1">Recovery Codes</h3>
                  <p className="text-shadow-400 text-sm">Download backup codes for account recovery</p>
                </div>
                <button className="btn-secondary text-sm">Generate Codes</button>
              </div>
            </div>

            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-6">Sessions</h2>
              
              <div className="space-y-4">
                <div className="flex items-center justify-between p-4 bg-shadow-800/50 rounded-lg">
                  <div className="flex items-center space-x-4">
                    <div className="w-10 h-10 bg-green-500/20 rounded-lg flex items-center justify-center">
                      <Globe className="w-5 h-5 text-green-500" />
                    </div>
                    <div>
                      <h3 className="font-medium text-white">Chrome on macOS</h3>
                      <p className="text-shadow-400 text-sm">San Francisco, US • Current session</p>
                    </div>
                  </div>
                  <span className="text-green-500 text-sm">Active</span>
                </div>
                <div className="flex items-center justify-between p-4 bg-shadow-800/50 rounded-lg">
                  <div className="flex items-center space-x-4">
                    <div className="w-10 h-10 bg-shadow-700 rounded-lg flex items-center justify-center">
                      <Globe className="w-5 h-5 text-shadow-400" />
                    </div>
                    <div>
                      <h3 className="font-medium text-white">Game Client</h3>
                      <p className="text-shadow-400 text-sm">San Francisco, US • 2 hours ago</p>
                    </div>
                  </div>
                  <button className="text-red-400 hover:text-red-300 text-sm">Revoke</button>
                </div>
              </div>
            </div>
          </Tabs.Content>

          {/* Notifications Tab */}
          <Tabs.Content value="notifications" className="space-y-6">
            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-6">Email Notifications</h2>
              
              <div className="space-y-4">
                {[
                  { title: 'Security Alerts', desc: 'Get notified about suspicious activity' },
                  { title: 'Level Up', desc: 'When your character reaches a new level' },
                  { title: 'Trade Completed', desc: 'When an item is bought or sold' },
                  { title: 'Guild Updates', desc: 'News and events from your guild' },
                  { title: 'Newsletter', desc: 'Weekly updates and announcements' },
                ].map((item) => (
                  <div key={item.title} className="flex items-center justify-between p-4 bg-shadow-800/50 rounded-lg">
                    <div>
                      <h3 className="font-medium text-white">{item.title}</h3>
                      <p className="text-shadow-400 text-sm">{item.desc}</p>
                    </div>
                    <Switch.Root 
                      defaultChecked={item.title !== 'Newsletter'}
                      className="w-11 h-6 bg-shadow-700 rounded-full relative data-[state=checked]:bg-accent-500 transition-colors"
                    >
                      <Switch.Thumb className="block w-5 h-5 bg-white rounded-full shadow-lg transition-transform translate-x-0.5 data-[state=checked]:translate-x-5" />
                    </Switch.Root>
                  </div>
                ))}
              </div>
            </div>
          </Tabs.Content>

          {/* Preferences Tab */}
          <Tabs.Content value="preferences" className="space-y-6">
            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-6">Display Preferences</h2>
              
              <div className="space-y-6">
                <div>
                  <label className="block text-sm font-medium text-shadow-300 mb-2">
                    Language
                  </label>
                  <select className="input">
                    <option>English (US)</option>
                    <option>Português (BR)</option>
                    <option>Español</option>
                    <option>Polski</option>
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium text-shadow-300 mb-2">
                    Timezone
                  </label>
                  <select className="input">
                    <option>UTC-08:00 Pacific Time</option>
                    <option>UTC-05:00 Eastern Time</option>
                    <option>UTC+00:00 GMT</option>
                    <option>UTC+01:00 Central European Time</option>
                  </select>
                </div>
              </div>
            </div>

            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-6">Privacy</h2>
              
              <div className="space-y-4">
                <div className="flex items-center justify-between p-4 bg-shadow-800/50 rounded-lg">
                  <div>
                    <h3 className="font-medium text-white">Show online status</h3>
                    <p className="text-shadow-400 text-sm">Let others see when you're online</p>
                  </div>
                  <Switch.Root defaultChecked className="w-11 h-6 bg-shadow-700 rounded-full relative data-[state=checked]:bg-accent-500 transition-colors">
                    <Switch.Thumb className="block w-5 h-5 bg-white rounded-full shadow-lg transition-transform translate-x-0.5 data-[state=checked]:translate-x-5" />
                  </Switch.Root>
                </div>
                <div className="flex items-center justify-between p-4 bg-shadow-800/50 rounded-lg">
                  <div>
                    <h3 className="font-medium text-white">Public profile</h3>
                    <p className="text-shadow-400 text-sm">Allow others to view your profile</p>
                  </div>
                  <Switch.Root defaultChecked className="w-11 h-6 bg-shadow-700 rounded-full relative data-[state=checked]:bg-accent-500 transition-colors">
                    <Switch.Thumb className="block w-5 h-5 bg-white rounded-full shadow-lg transition-transform translate-x-0.5 data-[state=checked]:translate-x-5" />
                  </Switch.Root>
                </div>
              </div>
            </div>
          </Tabs.Content>
        </Tabs.Root>
      </motion.div>
    </div>
  )
}

