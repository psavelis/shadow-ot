'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import {
  Shield, Key, Smartphone, Monitor, Globe, AlertTriangle,
  Lock, CheckCircle, XCircle, Eye, EyeOff, RefreshCw,
  Trash2, Download, LogOut, History, ChevronRight, Copy,
  Check, Link2, Unlink, Fingerprint, Plus, KeyRound, Usb,
  Repeat, Server, Zap, ArrowRightLeft
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'
import * as Tabs from '@radix-ui/react-tabs'
import Image from 'next/image'

const sessions = [
  { id: '1', device: 'MacBook Pro', browser: 'Chrome 120', ip: '192.168.1.1', location: 'San Francisco, US', lastActive: 'Now', current: true },
  { id: '2', device: 'iPhone 15 Pro', browser: 'Safari Mobile', ip: '192.168.1.45', location: 'San Francisco, US', lastActive: '2 hours ago', current: false },
  { id: '3', device: 'Windows PC', browser: 'Firefox 121', ip: '45.67.89.123', location: 'Los Angeles, US', lastActive: '2 days ago', current: false },
]

const activityLog = [
  { action: 'Password changed', ip: '192.168.1.1', location: 'San Francisco, US', time: '2024-12-04 14:32' },
  { action: 'New login from Chrome', ip: '192.168.1.1', location: 'San Francisco, US', time: '2024-12-04 10:15' },
  { action: '2FA enabled', ip: '192.168.1.1', location: 'San Francisco, US', time: '2024-12-03 18:45' },
  { action: 'Email verification completed', ip: '192.168.1.45', location: 'San Francisco, US', time: '2024-12-01 09:20' },
  { action: 'Account created', ip: '45.67.89.123', location: 'Los Angeles, US', time: '2024-11-28 16:00' },
]

const backupCodes = ['ABCD-1234', 'EFGH-5678', 'IJKL-9012', 'MNOP-3456', 'QRST-7890', 'UVWX-1234', 'YZAB-5678', 'CDEF-9012']

// Mock security keys data
const mockSecurityKeys = [
  { id: '1', name: 'YubiKey 5 NFC', type: 'yubikey', addedAt: '2024-11-15', lastUsed: '2024-12-04' },
  { id: '2', name: 'Backup Key', type: 'fido2', addedAt: '2024-10-20', lastUsed: '2024-11-28' },
]

export default function SecurityPage() {
  const [showChangePassword, setShowChangePassword] = useState(false)
  const [show2FASetup, setShow2FASetup] = useState(false)
  const [showBackupCodes, setShowBackupCodes] = useState(false)
  const [showAddSecurityKey, setShowAddSecurityKey] = useState(false)
  const [showPassword, setShowPassword] = useState(false)
  const [passwordForm, setPasswordForm] = useState({ current: '', new: '', confirm: '' })
  const [twoFACode, setTwoFACode] = useState('')
  const [twoFAEnabled, setTwoFAEnabled] = useState(false)
  const [copiedCode, setCopiedCode] = useState<string | null>(null)
  const [securityKeys, setSecurityKeys] = useState(mockSecurityKeys)
  const [newKeyName, setNewKeyName] = useState('')
  const [isRegisteringKey, setIsRegisteringKey] = useState(false)
  const [ssoEnabled, setSsoEnabled] = useState(true)
  const [ssoRealms, setSsoRealms] = useState([
    { id: 'nova', name: 'Nova', enabled: true, lastSync: '2 min ago' },
    { id: 'eldoria', name: 'Eldoria', enabled: true, lastSync: '5 min ago' },
    { id: 'shadowlands', name: 'Shadowlands', enabled: false, lastSync: 'Never' },
    { id: 'ancient', name: 'Ancient Kingdoms', enabled: true, lastSync: '1 hour ago' },
  ])

  const copyCode = (code: string) => {
    navigator.clipboard.writeText(code)
    setCopiedCode(code)
    setTimeout(() => setCopiedCode(null), 2000)
  }

  return (
    <div className="space-y-6 max-w-4xl">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <h1 className="text-2xl font-bold text-white mb-1">Security</h1>
        <p className="text-slate-400">Manage your account security and privacy settings</p>
      </motion.div>

      {/* Security Score */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="bg-gradient-to-r from-emerald-500/10 via-emerald-500/5 to-transparent border border-emerald-500/30 rounded-xl p-6"
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <div className="w-16 h-16 bg-emerald-500/20 rounded-xl flex items-center justify-center">
              <Shield className="w-8 h-8 text-emerald-400" />
            </div>
            <div>
              <h2 className="text-xl font-bold text-white">Security Score: Good</h2>
              <p className="text-slate-400 text-sm">Your account is well protected</p>
            </div>
          </div>
          <div className="text-right">
            <p className="text-4xl font-bold text-emerald-400">85%</p>
            <p className="text-slate-500 text-sm">Score</p>
          </div>
        </div>
        
        <div className="mt-4 grid grid-cols-4 gap-4">
          {[
            { label: 'Password', status: 'strong', icon: Key },
            { label: '2FA', status: twoFAEnabled ? 'enabled' : 'disabled', icon: Smartphone },
            { label: 'Email', status: 'verified', icon: CheckCircle },
            { label: 'Recovery', status: 'set', icon: RefreshCw },
          ].map((item, idx) => (
            <div key={idx} className="bg-slate-900/50 rounded-lg p-3 flex items-center gap-3">
              <item.icon className={`w-5 h-5 ${
                item.status === 'strong' || item.status === 'enabled' || item.status === 'verified' || item.status === 'set'
                  ? 'text-emerald-400'
                  : 'text-amber-400'
              }`} />
              <div>
                <p className="text-white text-sm font-medium">{item.label}</p>
                <p className={`text-xs capitalize ${
                  item.status === 'strong' || item.status === 'enabled' || item.status === 'verified' || item.status === 'set'
                    ? 'text-emerald-400'
                    : 'text-amber-400'
                }`}>{item.status}</p>
              </div>
            </div>
          ))}
        </div>
      </motion.div>

      {/* Tabs */}
      <Tabs.Root defaultValue="password">
        <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1 mb-6 overflow-x-auto">
          {[
            { id: 'password', label: 'Password', icon: Key },
            { id: '2fa', label: 'Two-Factor', icon: Smartphone },
            { id: 'securitykeys', label: 'Security Keys', icon: Fingerprint },
            { id: 'sso', label: 'Cross-Realm SSO', icon: ArrowRightLeft },
            { id: 'linked', label: 'Linked Accounts', icon: Link2 },
            { id: 'sessions', label: 'Sessions', icon: Monitor },
            { id: 'activity', label: 'Activity', icon: History },
          ].map(tab => (
            <Tabs.Trigger
              key={tab.id}
              value={tab.id}
              className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400 data-[state=inactive]:hover:text-white"
            >
              <tab.icon className="w-4 h-4" />
              {tab.label}
            </Tabs.Trigger>
          ))}
        </Tabs.List>

        {/* Password Tab */}
        <Tabs.Content value="password">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
          >
            <h3 className="text-white font-medium mb-6 flex items-center gap-2">
              <Key className="w-5 h-5 text-amber-400" />
              Change Password
            </h3>

            <div className="space-y-4 max-w-md">
              <div>
                <label className="block text-sm text-slate-400 mb-2">Current Password</label>
                <div className="relative">
                  <input
                    type={showPassword ? 'text' : 'password'}
                    value={passwordForm.current}
                    onChange={e => setPasswordForm({ ...passwordForm, current: e.target.value })}
                    className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2.5 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
                    placeholder="Enter current password"
                  />
                  <button
                    type="button"
                    onClick={() => setShowPassword(!showPassword)}
                    className="absolute right-3 top-1/2 -translate-y-1/2 text-slate-500 hover:text-white"
                  >
                    {showPassword ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                  </button>
                </div>
              </div>

              <div>
                <label className="block text-sm text-slate-400 mb-2">New Password</label>
                <input
                  type={showPassword ? 'text' : 'password'}
                  value={passwordForm.new}
                  onChange={e => setPasswordForm({ ...passwordForm, new: e.target.value })}
                  className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2.5 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
                  placeholder="Enter new password"
                />
              </div>

              <div>
                <label className="block text-sm text-slate-400 mb-2">Confirm New Password</label>
                <input
                  type={showPassword ? 'text' : 'password'}
                  value={passwordForm.confirm}
                  onChange={e => setPasswordForm({ ...passwordForm, confirm: e.target.value })}
                  className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2.5 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
                  placeholder="Confirm new password"
                />
              </div>

              <button className="w-full py-2.5 bg-gradient-to-r from-amber-500 to-orange-500 text-white rounded-lg font-medium hover:from-amber-400 hover:to-orange-400 transition">
                Update Password
              </button>
            </div>

            <div className="mt-6 pt-6 border-t border-slate-700">
              <p className="text-slate-400 text-sm mb-4">Password last changed: 30 days ago</p>
              <div className="flex items-center gap-2 text-xs text-slate-500">
                <AlertTriangle className="w-4 h-4 text-amber-400" />
                Use a strong password with at least 8 characters, including uppercase, lowercase, numbers, and symbols.
              </div>
            </div>
          </motion.div>
        </Tabs.Content>

        {/* 2FA Tab */}
        <Tabs.Content value="2fa">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-6"
          >
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <div className="flex items-center justify-between mb-6">
                <div className="flex items-center gap-4">
                  <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${
                    twoFAEnabled ? 'bg-emerald-500/20' : 'bg-slate-700/50'
                  }`}>
                    <Smartphone className={`w-6 h-6 ${twoFAEnabled ? 'text-emerald-400' : 'text-slate-400'}`} />
                  </div>
                  <div>
                    <h3 className="text-white font-medium">Two-Factor Authentication</h3>
                    <p className="text-slate-400 text-sm">
                      {twoFAEnabled ? 'Your account is protected with 2FA' : 'Add an extra layer of security'}
                    </p>
                  </div>
                </div>
                <button
                  onClick={() => {
                    if (twoFAEnabled) {
                      setTwoFAEnabled(false)
                    } else {
                      setShow2FASetup(true)
                    }
                  }}
                  className={`px-4 py-2 rounded-lg font-medium transition ${
                    twoFAEnabled
                      ? 'bg-red-500/20 text-red-400 hover:bg-red-500/30'
                      : 'bg-amber-500 text-white hover:bg-amber-400'
                  }`}
                >
                  {twoFAEnabled ? 'Disable' : 'Enable'}
                </button>
              </div>

              {twoFAEnabled && (
                <div className="border-t border-slate-700 pt-6 space-y-4">
                  <button
                    onClick={() => setShowBackupCodes(true)}
                    className="w-full flex items-center justify-between p-4 bg-slate-900/50 rounded-lg hover:bg-slate-800/50 transition"
                  >
                    <div className="flex items-center gap-3">
                      <Download className="w-5 h-5 text-slate-400" />
                      <div className="text-left">
                        <p className="text-white font-medium">Backup Codes</p>
                        <p className="text-slate-500 text-sm">View or regenerate your backup codes</p>
                      </div>
                    </div>
                    <ChevronRight className="w-5 h-5 text-slate-500" />
                  </button>
                </div>
              )}
            </div>
          </motion.div>
        </Tabs.Content>

        {/* Security Keys Tab (FIDO2/YubiKey) */}
        <Tabs.Content value="securitykeys">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-6"
          >
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <div className="flex items-center justify-between mb-6">
                <div className="flex items-center gap-4">
                  <div className="w-12 h-12 bg-purple-500/20 rounded-xl flex items-center justify-center">
                    <Fingerprint className="w-6 h-6 text-purple-400" />
                  </div>
                  <div>
                    <h3 className="text-white font-medium">Hardware Security Keys</h3>
                    <p className="text-slate-400 text-sm">
                      Use YubiKey, FIDO2, or other hardware keys for enhanced security
                    </p>
                  </div>
                </div>
                <button
                  onClick={() => setShowAddSecurityKey(true)}
                  className="flex items-center gap-2 px-4 py-2 bg-purple-500 text-white rounded-lg hover:bg-purple-400 transition font-medium"
                >
                  <Plus className="w-4 h-4" />
                  Add Key
                </button>
              </div>

              {/* Security Keys List */}
              {securityKeys.length > 0 ? (
                <div className="space-y-3">
                  {securityKeys.map((key) => (
                    <div key={key.id} className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg hover:bg-slate-800/50 transition">
                      <div className="flex items-center gap-4">
                        <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                          key.type === 'yubikey' ? 'bg-green-500/20' : 'bg-blue-500/20'
                        }`}>
                          {key.type === 'yubikey' ? (
                            <Usb className={`w-5 h-5 ${key.type === 'yubikey' ? 'text-green-400' : 'text-blue-400'}`} />
                          ) : (
                            <KeyRound className="w-5 h-5 text-blue-400" />
                          )}
                        </div>
                        <div>
                          <p className="text-white font-medium">{key.name}</p>
                          <p className="text-slate-500 text-sm">
                            Added {key.addedAt} • Last used {key.lastUsed}
                          </p>
                        </div>
                      </div>
                      <div className="flex items-center gap-2">
                        <span className={`px-2 py-0.5 text-xs rounded ${
                          key.type === 'yubikey' 
                            ? 'bg-green-500/20 text-green-400' 
                            : 'bg-blue-500/20 text-blue-400'
                        }`}>
                          {key.type === 'yubikey' ? 'YubiKey' : 'FIDO2'}
                        </span>
                        <button 
                          onClick={() => setSecurityKeys(keys => keys.filter(k => k.id !== key.id))}
                          className="p-2 text-red-400 hover:bg-red-500/20 rounded-lg transition"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="text-center py-8 border-2 border-dashed border-slate-700 rounded-xl">
                  <Fingerprint className="w-12 h-12 text-slate-600 mx-auto mb-3" />
                  <p className="text-slate-400 mb-2">No security keys registered</p>
                  <p className="text-slate-500 text-sm mb-4">
                    Add a hardware security key for the most secure authentication
                  </p>
                  <button
                    onClick={() => setShowAddSecurityKey(true)}
                    className="inline-flex items-center gap-2 px-4 py-2 bg-purple-500/20 text-purple-400 rounded-lg hover:bg-purple-500/30 transition"
                  >
                    <Plus className="w-4 h-4" />
                    Register Your First Key
                  </button>
                </div>
              )}
            </div>

            {/* Info Cards */}
            <div className="grid md:grid-cols-2 gap-4">
              <div className="bg-gradient-to-br from-green-500/10 to-emerald-500/5 border border-green-500/30 rounded-xl p-4">
                <div className="flex items-center gap-3 mb-3">
                  <div className="w-8 h-8 bg-green-500/20 rounded-lg flex items-center justify-center">
                    <Usb className="w-4 h-4 text-green-400" />
                  </div>
                  <h4 className="text-white font-medium">YubiKey</h4>
                </div>
                <p className="text-slate-400 text-sm">
                  Industry-standard hardware keys with USB-A, USB-C, NFC, and Lightning support. 
                  Works offline with no batteries required.
                </p>
              </div>

              <div className="bg-gradient-to-br from-blue-500/10 to-cyan-500/5 border border-blue-500/30 rounded-xl p-4">
                <div className="flex items-center gap-3 mb-3">
                  <div className="w-8 h-8 bg-blue-500/20 rounded-lg flex items-center justify-center">
                    <KeyRound className="w-4 h-4 text-blue-400" />
                  </div>
                  <h4 className="text-white font-medium">FIDO2/WebAuthn</h4>
                </div>
                <p className="text-slate-400 text-sm">
                  Open standard for passwordless authentication. Compatible with Windows Hello, 
                  Touch ID, Android biometrics, and more.
                </p>
              </div>
            </div>
          </motion.div>
        </Tabs.Content>

        {/* SSO (Single Sign-On) Tab */}
        <Tabs.Content value="sso">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-6"
          >
            {/* SSO Overview */}
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <div className="flex items-center justify-between mb-6">
                <div className="flex items-center gap-4">
                  <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${
                    ssoEnabled ? 'bg-cyan-500/20' : 'bg-slate-700/50'
                  }`}>
                    <ArrowRightLeft className={`w-6 h-6 ${ssoEnabled ? 'text-cyan-400' : 'text-slate-400'}`} />
                  </div>
                  <div>
                    <h3 className="text-white font-medium">Cross-Realm Single Sign-On</h3>
                    <p className="text-slate-400 text-sm">
                      {ssoEnabled ? 'One login, access all realms instantly' : 'Enable SSO to access multiple realms'}
                    </p>
                  </div>
                </div>
                <button
                  onClick={() => setSsoEnabled(!ssoEnabled)}
                  className={`relative w-14 h-7 rounded-full transition ${
                    ssoEnabled ? 'bg-cyan-500' : 'bg-slate-700'
                  }`}
                >
                  <div className={`absolute top-1 w-5 h-5 bg-white rounded-full transition-all ${
                    ssoEnabled ? 'left-8' : 'left-1'
                  }`} />
                </button>
              </div>

              {ssoEnabled && (
                <div className="bg-gradient-to-r from-cyan-500/10 to-blue-500/5 border border-cyan-500/30 rounded-lg p-4 flex items-center gap-3">
                  <Zap className="w-5 h-5 text-cyan-400" />
                  <p className="text-cyan-200 text-sm">
                    SSO is active. You can seamlessly switch between enabled realms without re-authenticating.
                  </p>
                </div>
              )}
            </div>

            {/* Realm SSO Settings */}
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                <Server className="w-5 h-5 text-cyan-400" />
                Realm Access
              </h3>
              <p className="text-slate-400 text-sm mb-6">
                Choose which realms should be accessible with your single sign-on session.
              </p>

              <div className="space-y-3">
                {ssoRealms.map((realm) => (
                  <div key={realm.id} className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                    <div className="flex items-center gap-4">
                      <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                        realm.enabled ? 'bg-cyan-500/20' : 'bg-slate-700/50'
                      }`}>
                        <Globe className={`w-5 h-5 ${realm.enabled ? 'text-cyan-400' : 'text-slate-500'}`} />
                      </div>
                      <div>
                        <p className="text-white font-medium">{realm.name}</p>
                        <p className="text-slate-500 text-sm">
                          {realm.enabled ? `Last sync: ${realm.lastSync}` : 'SSO disabled for this realm'}
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center gap-3">
                      {realm.enabled && (
                        <span className="px-2 py-0.5 bg-cyan-500/20 text-cyan-400 text-xs rounded">Active</span>
                      )}
                      <button
                        onClick={() => setSsoRealms(realms => realms.map(r => 
                          r.id === realm.id ? { ...r, enabled: !r.enabled } : r
                        ))}
                        className={`relative w-11 h-6 rounded-full transition ${
                          realm.enabled ? 'bg-cyan-500' : 'bg-slate-700'
                        }`}
                      >
                        <div className={`absolute top-1 w-4 h-4 bg-white rounded-full transition-all ${
                          realm.enabled ? 'left-6' : 'left-1'
                        }`} />
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* SSO Info Cards */}
            <div className="grid md:grid-cols-3 gap-4">
              <div className="bg-gradient-to-br from-cyan-500/10 to-blue-500/5 border border-cyan-500/30 rounded-xl p-4">
                <div className="w-10 h-10 bg-cyan-500/20 rounded-lg flex items-center justify-center mb-3">
                  <Zap className="w-5 h-5 text-cyan-400" />
                </div>
                <h4 className="text-white font-medium mb-1">Instant Access</h4>
                <p className="text-slate-400 text-sm">
                  Switch between realms without logging in again.
                </p>
              </div>

              <div className="bg-gradient-to-br from-purple-500/10 to-pink-500/5 border border-purple-500/30 rounded-xl p-4">
                <div className="w-10 h-10 bg-purple-500/20 rounded-lg flex items-center justify-center mb-3">
                  <Shield className="w-5 h-5 text-purple-400" />
                </div>
                <h4 className="text-white font-medium mb-1">Secure Tokens</h4>
                <p className="text-slate-400 text-sm">
                  JWT tokens with realm-specific permissions.
                </p>
              </div>

              <div className="bg-gradient-to-br from-amber-500/10 to-orange-500/5 border border-amber-500/30 rounded-xl p-4">
                <div className="w-10 h-10 bg-amber-500/20 rounded-lg flex items-center justify-center mb-3">
                  <Repeat className="w-5 h-5 text-amber-400" />
                </div>
                <h4 className="text-white font-medium mb-1">Auto Sync</h4>
                <p className="text-slate-400 text-sm">
                  Session state syncs across all enabled realms.
                </p>
              </div>
            </div>
          </motion.div>
        </Tabs.Content>

        {/* Sessions Tab */}
        <Tabs.Content value="sessions">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
          >
            <div className="p-4 border-b border-slate-700 flex items-center justify-between">
              <h3 className="text-white font-medium">Active Sessions</h3>
              <button className="text-red-400 text-sm hover:text-red-300 transition flex items-center gap-1">
                <LogOut className="w-4 h-4" />
                Sign out all other sessions
              </button>
            </div>
            <div className="divide-y divide-slate-700/50">
              {sessions.map((session) => (
                <div key={session.id} className="p-4 flex items-center justify-between hover:bg-slate-700/30 transition">
                  <div className="flex items-center gap-4">
                    <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                      session.device.includes('iPhone') ? 'bg-blue-500/20' :
                      session.device.includes('Mac') ? 'bg-slate-700/50' :
                      'bg-purple-500/20'
                    }`}>
                      {session.device.includes('iPhone') ? (
                        <Smartphone className="w-5 h-5 text-blue-400" />
                      ) : (
                        <Monitor className="w-5 h-5 text-slate-400" />
                      )}
                    </div>
                    <div>
                      <div className="flex items-center gap-2">
                        <p className="text-white font-medium">{session.device}</p>
                        {session.current && (
                          <span className="px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded">Current</span>
                        )}
                      </div>
                      <p className="text-slate-500 text-sm">{session.browser} • {session.location}</p>
                      <p className="text-slate-600 text-xs">{session.ip} • {session.lastActive}</p>
                    </div>
                  </div>
                  {!session.current && (
                    <button className="p-2 text-red-400 hover:bg-red-500/20 rounded-lg transition">
                      <Trash2 className="w-4 h-4" />
                    </button>
                  )}
                </div>
              ))}
            </div>
          </motion.div>
        </Tabs.Content>

        {/* Linked Accounts Tab */}
        <Tabs.Content value="linked">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-4"
          >
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <h3 className="text-white font-medium mb-6 flex items-center gap-2">
                <Link2 className="w-5 h-5 text-amber-400" />
                Social Login Accounts
              </h3>
              <p className="text-slate-400 text-sm mb-6">
                Link your social accounts for faster login. You can use any linked account to sign in.
              </p>
              
              <div className="space-y-3">
                {/* Google */}
                <div className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                  <div className="flex items-center gap-4">
                    <div className="w-10 h-10 bg-white/10 rounded-lg flex items-center justify-center">
                      <svg className="w-5 h-5" viewBox="0 0 24 24">
                        <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                        <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                        <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                        <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
                      </svg>
                    </div>
                    <div>
                      <p className="text-white font-medium">Google</p>
                      <p className="text-emerald-400 text-sm">Connected as john.doe@gmail.com</p>
                    </div>
                  </div>
                  <button className="flex items-center gap-2 px-3 py-1.5 bg-red-500/20 text-red-400 rounded-lg hover:bg-red-500/30 transition text-sm">
                    <Unlink className="w-4 h-4" />
                    Unlink
                  </button>
                </div>

                {/* Discord */}
                <div className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                  <div className="flex items-center gap-4">
                    <div className="w-10 h-10 bg-[#5865F2]/20 rounded-lg flex items-center justify-center">
                      <svg className="w-5 h-5" viewBox="0 0 24 24" fill="#5865F2">
                        <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"/>
                      </svg>
                    </div>
                    <div>
                      <p className="text-white font-medium">Discord</p>
                      <p className="text-emerald-400 text-sm">Connected as ShadowMaster#1234</p>
                    </div>
                  </div>
                  <button className="flex items-center gap-2 px-3 py-1.5 bg-red-500/20 text-red-400 rounded-lg hover:bg-red-500/30 transition text-sm">
                    <Unlink className="w-4 h-4" />
                    Unlink
                  </button>
                </div>

                {/* Twitch - Not connected */}
                <div className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                  <div className="flex items-center gap-4">
                    <div className="w-10 h-10 bg-slate-700/50 rounded-lg flex items-center justify-center">
                      <svg className="w-5 h-5" viewBox="0 0 24 24" fill="#9146FF">
                        <path d="M11.571 4.714h1.715v5.143H11.57zm4.715 0H18v5.143h-1.714zM6 0L1.714 4.286v15.428h5.143V24l4.286-4.286h3.428L22.286 12V0zm14.571 11.143l-3.428 3.428h-3.429l-3 3v-3H6.857V1.714h13.714z"/>
                      </svg>
                    </div>
                    <div>
                      <p className="text-white font-medium">Twitch</p>
                      <p className="text-slate-500 text-sm">Not connected</p>
                    </div>
                  </div>
                  <button className="flex items-center gap-2 px-3 py-1.5 bg-[#9146FF]/20 text-[#9146FF] rounded-lg hover:bg-[#9146FF]/30 transition text-sm">
                    <Link2 className="w-4 h-4" />
                    Link
                  </button>
                </div>
              </div>
            </div>

            {/* Wallet Connections */}
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <h3 className="text-white font-medium mb-6 flex items-center gap-2">
                <svg className="w-5 h-5 text-purple-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                  <path d="M21 12V7H5a2 2 0 0 1 0-4h14v4"/>
                  <path d="M3 5v14a2 2 0 0 0 2 2h16v-5"/>
                  <path d="M18 12a2 2 0 0 0 0 4h4v-4h-4z"/>
                </svg>
                Linked Wallets
              </h3>
              <p className="text-slate-400 text-sm mb-6">
                Connect your Web3 wallets for NFT management and secure authentication.
              </p>
              
              <div className="space-y-3">
                <div className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                  <div className="flex items-center gap-4">
                    <div className="w-10 h-10 bg-orange-500/20 rounded-lg flex items-center justify-center">
                      <svg className="w-5 h-5" viewBox="0 0 35 33" fill="none">
                        <path d="M32.96 1L19.72 10.89l2.45-5.8L32.96 1z" fill="#E17726"/>
                        <path d="M2.04 1l13.1 9.98-2.32-5.89L2.04 1z" fill="#E27625"/>
                      </svg>
                    </div>
                    <div>
                      <p className="text-white font-medium">MetaMask</p>
                      <p className="text-emerald-400 text-sm font-mono">0x1234...5678</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded">Primary</span>
                    <button className="flex items-center gap-2 px-3 py-1.5 bg-red-500/20 text-red-400 rounded-lg hover:bg-red-500/30 transition text-sm">
                      <Unlink className="w-4 h-4" />
                    </button>
                  </div>
                </div>

                <button className="w-full flex items-center justify-center gap-2 p-4 border-2 border-dashed border-slate-700 rounded-lg hover:border-purple-500/50 hover:bg-purple-500/5 transition text-slate-400 hover:text-purple-400">
                  <Link2 className="w-5 h-5" />
                  Connect Another Wallet
                </button>
              </div>
            </div>
          </motion.div>
        </Tabs.Content>

        {/* Activity Log Tab */}
        <Tabs.Content value="activity">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
          >
            <div className="p-4 border-b border-slate-700">
              <h3 className="text-white font-medium">Recent Security Activity</h3>
            </div>
            <div className="divide-y divide-slate-700/50">
              {activityLog.map((log, idx) => (
                <div key={idx} className="p-4 flex items-center gap-4 hover:bg-slate-700/30 transition">
                  <div className="w-10 h-10 bg-slate-700/50 rounded-lg flex items-center justify-center">
                    <History className="w-5 h-5 text-slate-400" />
                  </div>
                  <div className="flex-1">
                    <p className="text-white">{log.action}</p>
                    <p className="text-slate-500 text-sm">{log.ip} • {log.location}</p>
                  </div>
                  <p className="text-slate-500 text-sm">{log.time}</p>
                </div>
              ))}
            </div>
          </motion.div>
        </Tabs.Content>
      </Tabs.Root>

      {/* 2FA Setup Dialog */}
      <Dialog.Root open={show2FASetup} onOpenChange={setShow2FASetup}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-2">Setup Two-Factor Authentication</Dialog.Title>
            <Dialog.Description className="text-slate-400 text-sm mb-6">
              Scan the QR code with your authenticator app, then enter the verification code.
            </Dialog.Description>

            <div className="text-center mb-6">
              <div className="w-48 h-48 bg-white rounded-xl mx-auto mb-4 flex items-center justify-center">
                {/* Placeholder for QR code */}
                <div className="text-slate-400 text-sm">QR Code</div>
              </div>
              <p className="text-slate-500 text-xs">Or enter this code manually:</p>
              <p className="font-mono text-white text-sm mt-1">ABCD EFGH IJKL MNOP</p>
            </div>

            <div className="mb-6">
              <label className="block text-sm text-slate-400 mb-2">Verification Code</label>
              <input
                type="text"
                value={twoFACode}
                onChange={e => setTwoFACode(e.target.value)}
                maxLength={6}
                className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white text-center text-2xl tracking-widest font-mono placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
                placeholder="000000"
              />
            </div>

            <div className="flex gap-3">
              <Dialog.Close className="flex-1 px-4 py-2.5 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                Cancel
              </Dialog.Close>
              <button
                onClick={() => {
                  setTwoFAEnabled(true)
                  setShow2FASetup(false)
                  setShowBackupCodes(true)
                }}
                disabled={twoFACode.length !== 6}
                className="flex-1 px-4 py-2.5 bg-amber-500 text-white rounded-lg hover:bg-amber-400 transition disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Verify & Enable
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>

      {/* Backup Codes Dialog */}
      <Dialog.Root open={showBackupCodes} onOpenChange={setShowBackupCodes}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-2">Backup Codes</Dialog.Title>
            <Dialog.Description className="text-slate-400 text-sm mb-6">
              Save these codes in a secure location. Each code can only be used once.
            </Dialog.Description>

            <div className="bg-slate-900/50 rounded-lg p-4 mb-6">
              <div className="grid grid-cols-2 gap-2">
                {backupCodes.map((code, idx) => (
                  <button
                    key={idx}
                    onClick={() => copyCode(code)}
                    className="flex items-center justify-between p-2 bg-slate-800/50 rounded font-mono text-sm text-white hover:bg-slate-700/50 transition"
                  >
                    {code}
                    {copiedCode === code ? (
                      <Check className="w-4 h-4 text-emerald-400" />
                    ) : (
                      <Copy className="w-4 h-4 text-slate-500" />
                    )}
                  </button>
                ))}
              </div>
            </div>

            <div className="flex gap-3">
              <button className="flex-1 flex items-center justify-center gap-2 px-4 py-2.5 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                <Download className="w-4 h-4" />
                Download
              </button>
              <Dialog.Close className="flex-1 px-4 py-2.5 bg-amber-500 text-white rounded-lg hover:bg-amber-400 transition">
                Done
              </Dialog.Close>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>

      {/* Add Security Key Dialog */}
      <Dialog.Root open={showAddSecurityKey} onOpenChange={setShowAddSecurityKey}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-2">Register Security Key</Dialog.Title>
            <Dialog.Description className="text-slate-400 text-sm mb-6">
              Add a hardware security key (YubiKey, FIDO2) for passwordless authentication.
            </Dialog.Description>

            {!isRegisteringKey ? (
              <>
                <div className="mb-6">
                  <label className="block text-sm text-slate-400 mb-2">Key Name</label>
                  <input
                    type="text"
                    value={newKeyName}
                    onChange={e => setNewKeyName(e.target.value)}
                    className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white placeholder-slate-500 focus:outline-none focus:border-purple-500/50"
                    placeholder="e.g., YubiKey 5 NFC, Backup Key"
                  />
                </div>

                <div className="bg-slate-900/50 rounded-lg p-4 mb-6">
                  <h4 className="text-white font-medium mb-3">Supported Keys</h4>
                  <div className="space-y-2 text-sm">
                    <div className="flex items-center gap-2 text-slate-400">
                      <CheckCircle className="w-4 h-4 text-green-400" />
                      YubiKey 5 Series (USB-A, USB-C, NFC)
                    </div>
                    <div className="flex items-center gap-2 text-slate-400">
                      <CheckCircle className="w-4 h-4 text-green-400" />
                      FIDO2/WebAuthn compatible keys
                    </div>
                    <div className="flex items-center gap-2 text-slate-400">
                      <CheckCircle className="w-4 h-4 text-green-400" />
                      Windows Hello, Touch ID, Face ID
                    </div>
                    <div className="flex items-center gap-2 text-slate-400">
                      <CheckCircle className="w-4 h-4 text-green-400" />
                      Android biometric authenticators
                    </div>
                  </div>
                </div>

                <div className="flex gap-3">
                  <Dialog.Close className="flex-1 px-4 py-2.5 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                    Cancel
                  </Dialog.Close>
                  <button
                    onClick={() => setIsRegisteringKey(true)}
                    disabled={!newKeyName.trim()}
                    className="flex-1 px-4 py-2.5 bg-purple-500 text-white rounded-lg hover:bg-purple-400 transition disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    Continue
                  </button>
                </div>
              </>
            ) : (
              <div className="text-center py-8">
                <div className="w-20 h-20 bg-purple-500/20 rounded-full flex items-center justify-center mx-auto mb-4 animate-pulse">
                  <Fingerprint className="w-10 h-10 text-purple-400" />
                </div>
                <h4 className="text-white font-medium mb-2">Touch Your Security Key</h4>
                <p className="text-slate-400 text-sm mb-6">
                  Insert your security key and touch the button or sensor to register it.
                </p>
                <div className="flex items-center justify-center gap-2 text-amber-400 text-sm mb-6">
                  <div className="w-2 h-2 bg-amber-400 rounded-full animate-bounce" />
                  <span>Waiting for security key...</span>
                </div>
                <button
                  onClick={() => {
                    // Simulate successful registration
                    const newKey = {
                      id: Date.now().toString(),
                      name: newKeyName,
                      type: 'fido2' as const,
                      addedAt: new Date().toISOString().split('T')[0],
                      lastUsed: 'Never'
                    }
                    setSecurityKeys(keys => [...keys, newKey])
                    setNewKeyName('')
                    setIsRegisteringKey(false)
                    setShowAddSecurityKey(false)
                  }}
                  className="text-slate-400 text-sm hover:text-white transition"
                >
                  Simulate Registration (Demo)
                </button>
                <div className="mt-4">
                  <button
                    onClick={() => {
                      setIsRegisteringKey(false)
                      setNewKeyName('')
                    }}
                    className="text-red-400 text-sm hover:text-red-300 transition"
                  >
                    Cancel
                  </button>
                </div>
              </div>
            )}
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}

