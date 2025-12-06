'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import {
  Shield, Key, Smartphone, Monitor, Globe, AlertTriangle,
  Lock, CheckCircle, XCircle, Eye, EyeOff, RefreshCw,
  Trash2, Download, LogOut, History, ChevronRight, Copy,
  Check
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

export default function SecurityPage() {
  const [showChangePassword, setShowChangePassword] = useState(false)
  const [show2FASetup, setShow2FASetup] = useState(false)
  const [showBackupCodes, setShowBackupCodes] = useState(false)
  const [showPassword, setShowPassword] = useState(false)
  const [passwordForm, setPasswordForm] = useState({ current: '', new: '', confirm: '' })
  const [twoFACode, setTwoFACode] = useState('')
  const [twoFAEnabled, setTwoFAEnabled] = useState(false)
  const [copiedCode, setCopiedCode] = useState<string | null>(null)

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
        <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1 mb-6">
          {[
            { id: 'password', label: 'Password', icon: Key },
            { id: '2fa', label: 'Two-Factor Auth', icon: Smartphone },
            { id: 'sessions', label: 'Sessions', icon: Monitor },
            { id: 'activity', label: 'Activity Log', icon: History },
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
    </div>
  )
}

