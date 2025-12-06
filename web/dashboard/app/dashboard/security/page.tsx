'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import {
  Shield, Key, Smartphone, Monitor, Globe, AlertTriangle,
  Lock, CheckCircle, XCircle, Eye, EyeOff, RefreshCw,
  Trash2, Download, LogOut, History, ChevronRight, Copy,
  Check, Link2, Fingerprint, Plus, Usb, Server, Zap, 
  ArrowRightLeft, Loader2
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'
import * as Tabs from '@radix-ui/react-tabs'
import {
  useSessions,
  useTwoFactor,
  useSecurityKeys,
  useActivityLog,
  useSSO,
  useChangePassword,
} from '@/shared/hooks/useAuth'

export default function SecurityPage() {
  const [showChangePassword, setShowChangePassword] = useState(false)
  const [show2FASetup, setShow2FASetup] = useState(false)
  const [showBackupCodes, setShowBackupCodes] = useState(false)
  const [showAddSecurityKey, setShowAddSecurityKey] = useState(false)
  const [showPassword, setShowPassword] = useState(false)
  const [passwordForm, setPasswordForm] = useState({ current: '', new: '', confirm: '' })
  const [twoFACode, setTwoFACode] = useState('')
  const [copiedCode, setCopiedCode] = useState<string | null>(null)
  const [newKeyName, setNewKeyName] = useState('')

  // Real hooks
  const { sessions, isLoading: sessionsLoading, revokeSession, revokeAllOtherSessions, error: sessionError } = useSessions()
  const { isEnabled: twoFAEnabled, setup, initSetup, enable: enable2FA, regenerateBackupCodes, isLoading: twoFALoading, error: twoFAError } = useTwoFactor()
  const { keys: securityKeys, registerKey, deleteKey, isRegistering, isLoading: keysLoading, error: keysError } = useSecurityKeys()
  const { logs: activityLogs, isLoading: logsLoading } = useActivityLog(20)
  const { realms: ssoRealms, isEnabled: ssoEnabled, toggleRealm, toggleGlobal, isLoading: ssoLoading } = useSSO()
  const { changePassword, isLoading: passwordLoading, error: passwordError, success: passwordSuccess } = useChangePassword()

  const copyCode = (code: string) => {
    navigator.clipboard.writeText(code)
    setCopiedCode(code)
    setTimeout(() => setCopiedCode(null), 2000)
  }

  const handleChangePassword = async () => {
    if (passwordForm.new !== passwordForm.confirm) return
    const success = await changePassword(passwordForm.current, passwordForm.new)
    if (success) {
      setShowChangePassword(false)
      setPasswordForm({ current: '', new: '', confirm: '' })
    }
  }

  const handle2FASetup = async () => {
    if (!setup) {
      await initSetup()
    }
    setShow2FASetup(true)
  }

  const handle2FAEnable = async () => {
    if (!twoFACode) return
    const success = await enable2FA(twoFACode)
    if (success) {
      setShow2FASetup(false)
      setTwoFACode('')
    }
  }

  const handleRegisterSecurityKey = async () => {
    if (!newKeyName.trim()) return
    const success = await registerKey(newKeyName)
    if (success) {
      setShowAddSecurityKey(false)
      setNewKeyName('')
    }
  }

  // Calculate security score based on actual settings
  const calculateScore = () => {
    let score = 50 // Base score
    if (twoFAEnabled) score += 25
    if (securityKeys.length > 0) score += 15
    if (ssoEnabled) score += 10
    return Math.min(score, 100)
  }

  const securityScore = calculateScore()
  const scoreColor = securityScore >= 80 ? 'emerald' : securityScore >= 60 ? 'amber' : 'red'

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
        className={`bg-gradient-to-r from-${scoreColor}-500/10 via-${scoreColor}-500/5 to-transparent border border-${scoreColor}-500/30 rounded-xl p-6`}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <div className={`w-16 h-16 bg-${scoreColor}-500/20 rounded-xl flex items-center justify-center`}>
              <Shield className={`w-8 h-8 text-${scoreColor}-400`} />
            </div>
            <div>
              <h2 className="text-xl font-bold text-white">
                Security Score: {securityScore >= 80 ? 'Good' : securityScore >= 60 ? 'Fair' : 'Needs Improvement'}
              </h2>
              <p className="text-slate-400 text-sm">
                {securityScore >= 80 ? 'Your account is well protected' : 'Enable more security features'}
              </p>
            </div>
          </div>
          <div className="text-right">
            <p className={`text-4xl font-bold text-${scoreColor}-400`}>{securityScore}%</p>
            <p className="text-slate-500 text-sm">Score</p>
          </div>
        </div>
        
        <div className="mt-4 grid grid-cols-4 gap-4">
          {[
            { label: '2FA', enabled: twoFAEnabled, icon: Smartphone },
            { label: 'Security Keys', enabled: securityKeys.length > 0, icon: Key },
            { label: 'SSO', enabled: ssoEnabled, icon: Link2 },
            { label: 'Recent Activity', enabled: true, icon: History },
          ].map((item) => (
            <div key={item.label} className={`p-3 rounded-lg ${item.enabled ? 'bg-emerald-500/10' : 'bg-slate-800/50'}`}>
              <div className="flex items-center gap-2">
                <item.icon className={`w-4 h-4 ${item.enabled ? 'text-emerald-400' : 'text-slate-500'}`} />
                <span className={`text-sm ${item.enabled ? 'text-emerald-400' : 'text-slate-500'}`}>{item.label}</span>
              </div>
              <p className={`text-xs mt-1 ${item.enabled ? 'text-emerald-400' : 'text-slate-600'}`}>
                {item.enabled ? 'Enabled' : 'Disabled'}
              </p>
            </div>
          ))}
        </div>
      </motion.div>

      {/* Tabs */}
      <Tabs.Root defaultValue="sessions" className="space-y-6">
        <Tabs.List className="flex gap-2 border-b border-slate-700 pb-2 overflow-x-auto">
          {[
            { value: 'sessions', label: 'Sessions', icon: Monitor },
            { value: '2fa', label: 'Two-Factor', icon: Smartphone },
            { value: 'keys', label: 'Security Keys', icon: Key },
            { value: 'sso', label: 'Cross-Realm SSO', icon: ArrowRightLeft },
            { value: 'activity', label: 'Activity Log', icon: History },
            { value: 'password', label: 'Password', icon: Lock },
          ].map((tab) => (
            <Tabs.Trigger
              key={tab.value}
              value={tab.value}
              className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-slate-400 hover:text-white transition whitespace-nowrap data-[state=active]:text-white data-[state=active]:border-b-2 data-[state=active]:border-emerald-500"
            >
              <tab.icon className="w-4 h-4" />
              {tab.label}
            </Tabs.Trigger>
          ))}
        </Tabs.List>

        {/* Sessions Tab */}
        <Tabs.Content value="sessions">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl"
          >
            <div className="p-4 border-b border-slate-700 flex items-center justify-between">
              <h3 className="text-white font-bold flex items-center gap-2">
                <Monitor className="w-5 h-5 text-blue-400" />
                Active Sessions
              </h3>
              <button
                onClick={() => revokeAllOtherSessions()}
                className="text-sm text-red-400 hover:text-red-300 transition"
              >
                Revoke All Other Sessions
              </button>
            </div>
            
            {sessionsLoading ? (
              <div className="p-8 flex items-center justify-center">
                <Loader2 className="w-8 h-8 animate-spin text-blue-400" />
              </div>
            ) : sessions.length > 0 ? (
              <div className="divide-y divide-slate-700/50">
                {sessions.map((session) => (
                  <div key={session.id} className="p-4 flex items-center justify-between hover:bg-slate-700/30 transition">
                    <div className="flex items-center gap-4">
                      <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${session.current ? 'bg-emerald-500/20' : 'bg-slate-700/50'}`}>
                        <Monitor className={`w-5 h-5 ${session.current ? 'text-emerald-400' : 'text-slate-400'}`} />
                      </div>
                      <div>
                        <div className="flex items-center gap-2">
                          <p className="text-white font-medium">{session.device}</p>
                          {session.current && (
                            <span className="px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded">Current</span>
                          )}
                        </div>
                        <p className="text-slate-400 text-sm">{session.browser}</p>
                        <p className="text-slate-500 text-xs">{session.ip} • {session.location}</p>
                      </div>
                    </div>
                    <div className="text-right">
                      <p className="text-slate-400 text-sm">{session.lastActive}</p>
                      {!session.current && (
                        <button
                          onClick={() => revokeSession(session.id)}
                          className="text-red-400 hover:text-red-300 text-sm mt-1"
                        >
                          Revoke
                        </button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="p-8 text-center text-slate-400">
                <Monitor className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>No active sessions</p>
              </div>
            )}
          </motion.div>
        </Tabs.Content>

        {/* 2FA Tab */}
        <Tabs.Content value="2fa">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
          >
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center gap-4">
                <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${twoFAEnabled ? 'bg-emerald-500/20' : 'bg-slate-700/50'}`}>
                  <Smartphone className={`w-6 h-6 ${twoFAEnabled ? 'text-emerald-400' : 'text-slate-400'}`} />
                </div>
                <div>
                  <h3 className="text-white font-bold">Two-Factor Authentication</h3>
                  <p className="text-slate-400 text-sm">
                    {twoFAEnabled ? 'Your account is protected with 2FA' : 'Add an extra layer of security'}
                  </p>
                </div>
              </div>
              <div className={`px-3 py-1 rounded-full text-sm ${twoFAEnabled ? 'bg-emerald-500/20 text-emerald-400' : 'bg-red-500/20 text-red-400'}`}>
                {twoFAEnabled ? 'Enabled' : 'Disabled'}
              </div>
            </div>

            {twoFAEnabled ? (
              <div className="space-y-4">
                <button
                  onClick={() => setShowBackupCodes(true)}
                  className="w-full flex items-center justify-between p-4 bg-slate-900/50 rounded-lg hover:bg-slate-700/50 transition"
                >
                  <div className="flex items-center gap-3">
                    <Download className="w-5 h-5 text-slate-400" />
                    <span className="text-white">View Backup Codes</span>
                  </div>
                  <ChevronRight className="w-5 h-5 text-slate-500" />
                </button>
              </div>
            ) : (
              <button
                onClick={handle2FASetup}
                disabled={twoFALoading}
                className="w-full py-3 bg-emerald-500 text-white rounded-lg font-medium hover:bg-emerald-600 transition disabled:opacity-50 flex items-center justify-center gap-2"
              >
                {twoFALoading ? (
                  <>
                    <Loader2 className="w-5 h-5 animate-spin" />
                    Setting up...
                  </>
                ) : (
                  <>
                    <Plus className="w-5 h-5" />
                    Enable Two-Factor Authentication
                  </>
                )}
              </button>
            )}
          </motion.div>
        </Tabs.Content>

        {/* Security Keys Tab */}
        <Tabs.Content value="keys">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl"
          >
            <div className="p-4 border-b border-slate-700 flex items-center justify-between">
              <h3 className="text-white font-bold flex items-center gap-2">
                <Fingerprint className="w-5 h-5 text-amber-400" />
                Security Keys (FIDO2/YubiKey)
              </h3>
              <button
                onClick={() => setShowAddSecurityKey(true)}
                className="flex items-center gap-2 px-3 py-1.5 bg-amber-500/20 text-amber-400 rounded-lg text-sm hover:bg-amber-500/30 transition"
              >
                <Plus className="w-4 h-4" />
                Add Key
              </button>
            </div>

            {keysLoading ? (
              <div className="p-8 flex items-center justify-center">
                <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
              </div>
            ) : securityKeys.length > 0 ? (
              <div className="divide-y divide-slate-700/50">
                {securityKeys.map((key) => (
                  <div key={key.id} className="p-4 flex items-center justify-between hover:bg-slate-700/30 transition">
                    <div className="flex items-center gap-4">
                      <div className="w-10 h-10 bg-amber-500/20 rounded-lg flex items-center justify-center">
                        {key.type === 'yubikey' ? <Usb className="w-5 h-5 text-amber-400" /> : <Key className="w-5 h-5 text-amber-400" />}
                      </div>
                      <div>
                        <p className="text-white font-medium">{key.name}</p>
                        <p className="text-slate-500 text-xs">
                          Added {new Date(key.addedAt).toLocaleDateString()} • Last used {new Date(key.lastUsed).toLocaleDateString()}
                        </p>
                      </div>
                    </div>
                    <button
                      onClick={() => deleteKey(key.id)}
                      className="p-2 text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg transition"
                    >
                      <Trash2 className="w-5 h-5" />
                    </button>
                  </div>
                ))}
              </div>
            ) : (
              <div className="p-8 text-center text-slate-400">
                <Key className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>No security keys registered</p>
                <p className="text-sm text-slate-500 mt-2">Add a hardware security key for stronger protection</p>
              </div>
            )}
          </motion.div>
        </Tabs.Content>

        {/* SSO Tab */}
        <Tabs.Content value="sso">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
          >
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center gap-4">
                <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${ssoEnabled ? 'bg-purple-500/20' : 'bg-slate-700/50'}`}>
                  <ArrowRightLeft className={`w-6 h-6 ${ssoEnabled ? 'text-purple-400' : 'text-slate-400'}`} />
                </div>
                <div>
                  <h3 className="text-white font-bold">Cross-Realm Single Sign-On</h3>
                  <p className="text-slate-400 text-sm">Seamlessly switch between realms without re-authenticating</p>
                </div>
              </div>
              <button
                onClick={() => toggleGlobal(!ssoEnabled)}
                disabled={ssoLoading}
                className={`relative w-12 h-6 rounded-full transition ${ssoEnabled ? 'bg-purple-500' : 'bg-slate-600'}`}
              >
                <div className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-all ${ssoEnabled ? 'left-7' : 'left-1'}`} />
              </button>
            </div>

            {ssoEnabled && ssoRealms.length > 0 && (
              <div className="space-y-3">
                {ssoRealms.map((realm) => (
                  <div key={realm.realmId} className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                    <div className="flex items-center gap-3">
                      <Server className="w-5 h-5 text-slate-400" />
                      <div>
                        <p className="text-white font-medium">{realm.realmName}</p>
                        <p className="text-slate-500 text-xs">
                          Last sync: {realm.lastSync || 'Never'}
                        </p>
                      </div>
                    </div>
                    <button
                      onClick={() => toggleRealm(realm.realmId, !realm.enabled)}
                      disabled={ssoLoading}
                      className={`relative w-10 h-5 rounded-full transition ${realm.enabled ? 'bg-purple-500' : 'bg-slate-600'}`}
                    >
                      <div className={`absolute top-0.5 w-4 h-4 rounded-full bg-white transition-all ${realm.enabled ? 'left-5' : 'left-0.5'}`} />
                    </button>
                  </div>
                ))}
              </div>
            )}
          </motion.div>
        </Tabs.Content>

        {/* Activity Log Tab */}
        <Tabs.Content value="activity">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl"
          >
            <div className="p-4 border-b border-slate-700">
              <h3 className="text-white font-bold flex items-center gap-2">
                <History className="w-5 h-5 text-cyan-400" />
                Recent Activity
              </h3>
            </div>

            {logsLoading ? (
              <div className="p-8 flex items-center justify-center">
                <Loader2 className="w-8 h-8 animate-spin text-cyan-400" />
              </div>
            ) : activityLogs.length > 0 ? (
              <div className="divide-y divide-slate-700/50">
                {activityLogs.map((log) => (
                  <div key={log.id} className="p-4 flex items-center justify-between hover:bg-slate-700/30 transition">
                    <div>
                      <p className="text-white">{log.action}</p>
                      <p className="text-slate-500 text-xs">{log.ip} • {log.location}</p>
                    </div>
                    <p className="text-slate-400 text-sm">
                      {new Date(log.timestamp).toLocaleString()}
                    </p>
                  </div>
                ))}
              </div>
            ) : (
              <div className="p-8 text-center text-slate-400">
                <History className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>No activity recorded</p>
              </div>
            )}
          </motion.div>
        </Tabs.Content>

        {/* Password Tab */}
        <Tabs.Content value="password">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
          >
            <div className="flex items-center gap-4 mb-6">
              <div className="w-12 h-12 bg-red-500/20 rounded-xl flex items-center justify-center">
                <Lock className="w-6 h-6 text-red-400" />
              </div>
              <div>
                <h3 className="text-white font-bold">Change Password</h3>
                <p className="text-slate-400 text-sm">Update your account password</p>
              </div>
            </div>

            <div className="space-y-4">
              <div>
                <label className="block text-sm text-slate-400 mb-2">Current Password</label>
                <div className="relative">
                  <input
                    type={showPassword ? 'text' : 'password'}
                    value={passwordForm.current}
                    onChange={(e) => setPasswordForm(p => ({ ...p, current: e.target.value }))}
                    className="w-full px-4 py-3 bg-slate-900/50 border border-slate-700 rounded-lg text-white placeholder-slate-500 focus:outline-none focus:border-red-500/50"
                    placeholder="Enter current password"
                  />
                  <button
                    type="button"
                    onClick={() => setShowPassword(!showPassword)}
                    className="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-white"
                  >
                    {showPassword ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
                  </button>
                </div>
              </div>
              <div>
                <label className="block text-sm text-slate-400 mb-2">New Password</label>
                <input
                  type="password"
                  value={passwordForm.new}
                  onChange={(e) => setPasswordForm(p => ({ ...p, new: e.target.value }))}
                  className="w-full px-4 py-3 bg-slate-900/50 border border-slate-700 rounded-lg text-white placeholder-slate-500 focus:outline-none focus:border-red-500/50"
                  placeholder="Enter new password"
                />
              </div>
              <div>
                <label className="block text-sm text-slate-400 mb-2">Confirm New Password</label>
                <input
                  type="password"
                  value={passwordForm.confirm}
                  onChange={(e) => setPasswordForm(p => ({ ...p, confirm: e.target.value }))}
                  className="w-full px-4 py-3 bg-slate-900/50 border border-slate-700 rounded-lg text-white placeholder-slate-500 focus:outline-none focus:border-red-500/50"
                  placeholder="Confirm new password"
                />
              </div>

              {passwordError && (
                <div className="p-3 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400 text-sm">
                  {passwordError}
                </div>
              )}

              {passwordSuccess && (
                <div className="p-3 bg-emerald-500/10 border border-emerald-500/30 rounded-lg text-emerald-400 text-sm">
                  Password changed successfully!
                </div>
              )}

              <button
                onClick={handleChangePassword}
                disabled={passwordLoading || !passwordForm.current || !passwordForm.new || passwordForm.new !== passwordForm.confirm}
                className="w-full py-3 bg-red-500 text-white rounded-lg font-medium hover:bg-red-600 transition disabled:opacity-50 flex items-center justify-center gap-2"
              >
                {passwordLoading ? (
                  <>
                    <Loader2 className="w-5 h-5 animate-spin" />
                    Updating...
                  </>
                ) : (
                  'Change Password'
                )}
              </button>
            </div>
          </motion.div>
        </Tabs.Content>
      </Tabs.Root>

      {/* 2FA Setup Dialog */}
      <Dialog.Root open={show2FASetup} onOpenChange={setShow2FASetup}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-md bg-slate-900 border border-slate-700 rounded-xl p-6 z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-4">
              Setup Two-Factor Authentication
            </Dialog.Title>
            
            {setup && (
              <div className="space-y-4">
                <div className="bg-white p-4 rounded-lg flex items-center justify-center">
                  <img src={setup.qrCode} alt="2FA QR Code" className="w-48 h-48" />
                </div>
                <p className="text-slate-400 text-sm text-center">
                  Scan this QR code with your authenticator app
                </p>
                <div className="p-3 bg-slate-800/50 rounded-lg">
                  <p className="text-slate-500 text-xs mb-1">Manual entry code:</p>
                  <p className="text-white font-mono text-sm">{setup.secret}</p>
                </div>
                <div>
                  <label className="block text-sm text-slate-400 mb-2">Enter verification code</label>
                  <input
                    type="text"
                    value={twoFACode}
                    onChange={(e) => setTwoFACode(e.target.value)}
                    placeholder="000000"
                    maxLength={6}
                    className="w-full px-4 py-3 bg-slate-800/50 border border-slate-700 rounded-lg text-white text-center font-mono text-2xl tracking-widest placeholder-slate-600 focus:outline-none focus:border-emerald-500/50"
                  />
                </div>
                <button
                  onClick={handle2FAEnable}
                  disabled={twoFALoading || twoFACode.length !== 6}
                  className="w-full py-3 bg-emerald-500 text-white rounded-lg font-medium hover:bg-emerald-600 transition disabled:opacity-50 flex items-center justify-center gap-2"
                >
                  {twoFALoading ? <Loader2 className="w-5 h-5 animate-spin" /> : 'Enable 2FA'}
                </button>
              </div>
            )}
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>

      {/* Add Security Key Dialog */}
      <Dialog.Root open={showAddSecurityKey} onOpenChange={setShowAddSecurityKey}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-md bg-slate-900 border border-slate-700 rounded-xl p-6 z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-4">
              Register Security Key
            </Dialog.Title>
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-slate-400 mb-2">Key Name</label>
                <input
                  type="text"
                  value={newKeyName}
                  onChange={(e) => setNewKeyName(e.target.value)}
                  placeholder="e.g., YubiKey 5"
                  className="w-full px-4 py-3 bg-slate-800/50 border border-slate-700 rounded-lg text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
                />
              </div>
              <p className="text-slate-400 text-sm">
                Insert your security key and click Register. You may need to touch the key when prompted.
              </p>
              <button
                onClick={handleRegisterSecurityKey}
                disabled={isRegistering || !newKeyName.trim()}
                className="w-full py-3 bg-amber-500 text-white rounded-lg font-medium hover:bg-amber-600 transition disabled:opacity-50 flex items-center justify-center gap-2"
              >
                {isRegistering ? (
                  <>
                    <Loader2 className="w-5 h-5 animate-spin" />
                    Waiting for key...
                  </>
                ) : (
                  <>
                    <Fingerprint className="w-5 h-5" />
                    Register Key
                  </>
                )}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}
