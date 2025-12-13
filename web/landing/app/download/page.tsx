'use client'

import { useState, useEffect } from 'react'
import { motion } from 'framer-motion'
import { 
  Download, 
  Apple, 
  Monitor, 
  Check,
  Cpu,
  HardDrive,
  MemoryStick,
  MonitorPlay,
  Gamepad2,
  Shield,
  Zap,
  Globe,
  ExternalLink,
  RefreshCw
} from 'lucide-react'

// Downloads service URL - configured via environment or defaults to k8s service
const DOWNLOADS_BASE_URL = process.env.NEXT_PUBLIC_DOWNLOADS_URL || '/downloads'

// Asset file mappings for each platform and version
const ASSET_FILES = {
  windows: {
    '12.85': 'shadow-ot-client-12.85-win64.exe',
    '10.98': 'shadow-ot-client-10.98-win64.exe',
    '8.60': 'shadow-ot-client-8.60-win64.exe',
  },
  macos: {
    '12.85': 'shadow-ot-client-12.85-macos-universal.dmg',
    '10.98': 'shadow-ot-client-10.98-macos-universal.dmg',
    '8.60': 'shadow-ot-client-8.60-macos-universal.dmg',
  },
  linux: {
    '12.85': 'shadow-ot-client-12.85-linux-x86_64.AppImage',
    '10.98': 'shadow-ot-client-10.98-linux-x86_64.AppImage',
    '8.60': 'shadow-ot-client-8.60-linux-x86_64.AppImage',
  },
} as const

type PlatformId = keyof typeof ASSET_FILES
type ClientVersion = '12.85' | '10.98' | '8.60'

const platforms = [
  {
    id: 'windows' as PlatformId,
    name: 'Windows',
    icon: Monitor,
    version: '2.1.0',
    size: '498 MB',
    requirements: '64-bit Windows 10 or later',
    recommended: true,
  },
  {
    id: 'macos' as PlatformId,
    name: 'macOS',
    icon: Apple,
    version: '2.1.0',
    size: '512 MB',
    requirements: 'macOS 11 (Big Sur) or later, Apple Silicon & Intel',
  },
  {
    id: 'linux' as PlatformId,
    name: 'Linux',
    icon: Monitor,
    version: '2.1.0',
    size: '478 MB',
    requirements: 'Ubuntu 20.04+, Fedora 35+, or equivalent',
  },
]

const clientVersions: { version: ClientVersion; name: string; features: string[]; recommended?: boolean }[] = [
  { version: '12.85', name: 'Modern', features: ['Store', 'Bosstiary', 'Prey', 'Analytics'], recommended: true },
  { version: '10.98', name: 'Popular', features: ['Classic UI', 'Most OT features', 'Stable'] },
  { version: '8.60', name: 'Classic', features: ['Nostalgic', 'Retro gameplay', '7.x protocol'] },
]

const features = [
  { icon: Zap, title: 'Auto-Update', description: 'Delta updates for fast patching' },
  { icon: Shield, title: 'Anti-Cheat', description: 'Integrated protection system' },
  { icon: Globe, title: 'Multi-Realm', description: 'Switch realms seamlessly' },
  { icon: Gamepad2, title: 'Bot Support', description: 'Built-in bot framework' },
]

const systemRequirements = {
  minimum: [
    { icon: Cpu, label: 'Processor', value: 'Intel Core i3 / AMD Ryzen 3' },
    { icon: MemoryStick, label: 'Memory', value: '4 GB RAM' },
    { icon: MonitorPlay, label: 'Graphics', value: 'OpenGL 3.3 compatible' },
    { icon: HardDrive, label: 'Storage', value: '2 GB available space' },
  ],
  recommended: [
    { icon: Cpu, label: 'Processor', value: 'Intel Core i5 / AMD Ryzen 5' },
    { icon: MemoryStick, label: 'Memory', value: '8 GB RAM' },
    { icon: MonitorPlay, label: 'Graphics', value: 'Dedicated GPU with 2GB VRAM' },
    { icon: HardDrive, label: 'Storage', value: '5 GB SSD space' },
  ],
}

// Helper to get download URL for platform and version
function getDownloadUrl(platform: PlatformId, version: ClientVersion): string {
  const filename = ASSET_FILES[platform][version]
  return `${DOWNLOADS_BASE_URL}/${filename}`
}

export default function DownloadPage() {
  const [selectedPlatform, setSelectedPlatform] = useState<PlatformId>('windows')
  const [selectedVersion, setSelectedVersion] = useState<ClientVersion>('12.85')
  const [isChecking, setIsChecking] = useState(false)
  const [downloadAvailable, setDownloadAvailable] = useState<boolean | null>(null)

  const currentPlatform = platforms.find(p => p.id === selectedPlatform)
  const downloadUrl = getDownloadUrl(selectedPlatform, selectedVersion)

  // Check if download is available
  useEffect(() => {
    const checkAvailability = async () => {
      setIsChecking(true)
      try {
        const response = await fetch(downloadUrl, { method: 'HEAD' })
        setDownloadAvailable(response.ok)
      } catch {
        setDownloadAvailable(false)
      } finally {
        setIsChecking(false)
      }
    }
    checkAvailability()
  }, [downloadUrl])

  return (
    <div className="min-h-screen bg-shadow-950 pt-24 pb-16">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <div className="inline-flex items-center space-x-2 bg-accent-500/10 border border-accent-500/30 rounded-full px-4 py-2 mb-6">
            <span className="relative flex h-2 w-2">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-500 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
            </span>
            <span className="text-green-400 text-sm font-medium">Version 2.1.0 Released</span>
          </div>
          <h1 className="font-display text-4xl md:text-5xl font-bold mb-4">
            Download <span className="gradient-text">Shadow OT</span>
          </h1>
          <p className="text-shadow-300 text-lg max-w-2xl mx-auto">
            Get the official Shadow OT client with auto-updates, multi-realm support, and blockchain integration built-in.
          </p>
        </motion.div>

        {/* Main Download Card */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="max-w-4xl mx-auto mb-16"
        >
          <div className="bg-gradient-to-br from-shadow-900 to-shadow-950 rounded-3xl border border-shadow-700 overflow-hidden">
            {/* Platform Tabs */}
            <div className="flex border-b border-shadow-700">
              {platforms.map((platform) => (
                <button
                  key={platform.id}
                  onClick={() => setSelectedPlatform(platform.id)}
                  className={`flex-1 flex items-center justify-center space-x-2 py-4 px-6 transition-all ${
                    selectedPlatform === platform.id
                      ? 'bg-shadow-800 text-white border-b-2 border-accent-500'
                      : 'text-shadow-400 hover:text-white hover:bg-shadow-800/50'
                  }`}
                >
                  <platform.icon className="w-5 h-5" />
                  <span className="font-medium">{platform.name}</span>
                  {platform.recommended && (
                    <span className="text-xs bg-accent-500/20 text-accent-400 px-2 py-0.5 rounded-full">
                      Recommended
                    </span>
                  )}
                </button>
              ))}
            </div>

            {/* Download Content */}
            <div className="p-8 md:p-12">
              <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-6 mb-8">
                <div>
                  <h2 className="font-display text-2xl font-bold mb-2">
                    Shadow OT for {currentPlatform?.name}
                  </h2>
                  <div className="flex flex-wrap items-center gap-4 text-sm text-shadow-400">
                    <span>Version {currentPlatform?.version}</span>
                    <span className="w-1 h-1 bg-shadow-600 rounded-full" />
                    <span>{currentPlatform?.size}</span>
                    <span className="w-1 h-1 bg-shadow-600 rounded-full" />
                    <span>{currentPlatform?.requirements}</span>
                  </div>
                </div>
                <a
                  href={downloadUrl}
                  className={`btn-primary flex items-center space-x-2 text-lg whitespace-nowrap ${
                    isChecking || downloadAvailable === false ? 'opacity-50 pointer-events-none' : ''
                  }`}
                  download
                >
                  {isChecking ? (
                    <RefreshCw className="w-5 h-5 animate-spin" />
                  ) : (
                    <Download className="w-5 h-5" />
                  )}
                  <span>{isChecking ? 'Checking...' : downloadAvailable === false ? 'Coming Soon' : 'Download Now'}</span>
                </a>
              </div>

              {/* Client Version Selection */}
              <div className="mb-8">
                <h3 className="text-sm font-medium text-shadow-400 mb-3">Select Client Version:</h3>
                <div className="grid md:grid-cols-3 gap-4">
                  {clientVersions.map((client) => (
                    <button
                      key={client.version}
                      onClick={() => setSelectedVersion(client.version)}
                      className={`relative p-4 rounded-xl border text-left transition-all ${
                        selectedVersion === client.version
                          ? 'bg-accent-500/10 border-accent-500/50'
                          : 'bg-shadow-800/50 border-shadow-700 hover:border-shadow-600'
                      }`}
                    >
                      {client.recommended && (
                        <span className="absolute -top-2 -right-2 text-xs bg-accent-500 text-white px-2 py-0.5 rounded-full">
                          Recommended
                        </span>
                      )}
                      <div className="flex items-center space-x-2 mb-2">
                        <span className="font-bold text-white">{client.version}</span>
                        <span className="text-shadow-500">•</span>
                        <span className="text-shadow-400">{client.name}</span>
                      </div>
                      <div className="flex flex-wrap gap-1">
                        {client.features.map((feature) => (
                          <span
                            key={feature}
                            className="text-xs bg-shadow-700 text-shadow-300 px-2 py-0.5 rounded"
                          >
                            {feature}
                          </span>
                        ))}
                      </div>
                    </button>
                  ))}
                </div>
              </div>

              {/* Quick Features */}
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                {features.map((feature) => (
                  <div key={feature.title} className="flex items-center space-x-3">
                    <div className="w-10 h-10 bg-shadow-800 rounded-lg flex items-center justify-center">
                      <feature.icon className="w-5 h-5 text-accent-500" />
                    </div>
                    <div>
                      <div className="text-white font-medium text-sm">{feature.title}</div>
                      <div className="text-shadow-500 text-xs">{feature.description}</div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </motion.div>

        {/* System Requirements */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="max-w-4xl mx-auto mb-16"
        >
          <h2 className="font-display text-2xl font-bold text-center mb-8">System Requirements</h2>
          <div className="grid md:grid-cols-2 gap-6">
            {/* Minimum */}
            <div className="bg-shadow-900/50 rounded-2xl border border-shadow-700 p-6">
              <h3 className="font-semibold text-white mb-4 flex items-center space-x-2">
                <span>Minimum</span>
              </h3>
              <div className="space-y-4">
                {systemRequirements.minimum.map((req) => (
                  <div key={req.label} className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-shadow-800 rounded-lg flex items-center justify-center">
                      <req.icon className="w-4 h-4 text-shadow-400" />
                    </div>
                    <div>
                      <div className="text-shadow-500 text-xs">{req.label}</div>
                      <div className="text-white text-sm">{req.value}</div>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Recommended */}
            <div className="bg-gradient-to-br from-accent-500/5 to-purple-500/5 rounded-2xl border border-accent-500/20 p-6">
              <h3 className="font-semibold text-white mb-4 flex items-center space-x-2">
                <span>Recommended</span>
                <span className="text-xs bg-accent-500/20 text-accent-400 px-2 py-0.5 rounded-full">
                  Best Experience
                </span>
              </h3>
              <div className="space-y-4">
                {systemRequirements.recommended.map((req) => (
                  <div key={req.label} className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-accent-500/10 rounded-lg flex items-center justify-center">
                      <req.icon className="w-4 h-4 text-accent-400" />
                    </div>
                    <div>
                      <div className="text-shadow-500 text-xs">{req.label}</div>
                      <div className="text-white text-sm">{req.value}</div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </motion.div>

        {/* Alternative Downloads */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
          className="max-w-4xl mx-auto"
        >
          <h2 className="font-display text-2xl font-bold text-center mb-8">Alternative Downloads</h2>
          <div className="bg-shadow-900/50 rounded-2xl border border-shadow-700 divide-y divide-shadow-700">
            <div className="p-4 flex items-center justify-between">
              <div className="flex items-center space-x-4">
                <div className="w-10 h-10 bg-shadow-800 rounded-lg flex items-center justify-center">
                  <Download className="w-5 h-5 text-shadow-400" />
                </div>
                <div>
                  <div className="text-white font-medium">Portable Version</div>
                  <div className="text-shadow-500 text-sm">No installation required, run from USB</div>
                </div>
              </div>
              <a 
                href={`${DOWNLOADS_BASE_URL}/shadow-ot-client-${selectedVersion}-portable.zip`}
                className="text-accent-500 hover:text-accent-400 text-sm font-medium flex items-center space-x-1"
                download
              >
                <span>Download</span>
                <ExternalLink className="w-4 h-4" />
              </a>
            </div>
            <div className="p-4 flex items-center justify-between">
              <div className="flex items-center space-x-4">
                <div className="w-10 h-10 bg-shadow-800 rounded-lg flex items-center justify-center">
                  <Download className="w-5 h-5 text-shadow-400" />
                </div>
                <div>
                  <div className="text-white font-medium">Full Asset Pack</div>
                  <div className="text-shadow-500 text-sm">High-resolution sprites and textures (2.1 GB)</div>
                </div>
              </div>
              <a 
                href={`${DOWNLOADS_BASE_URL}/shadow-ot-assets-full.zip`}
                className="text-accent-500 hover:text-accent-400 text-sm font-medium flex items-center space-x-1"
                download
              >
                <span>Download</span>
                <ExternalLink className="w-4 h-4" />
              </a>
            </div>
            <div className="p-4 flex items-center justify-between">
              <div className="flex items-center space-x-4">
                <div className="w-10 h-10 bg-shadow-800 rounded-lg flex items-center justify-center">
                  <Download className="w-5 h-5 text-shadow-400" />
                </div>
                <div>
                  <div className="text-white font-medium">Previous Versions</div>
                  <div className="text-shadow-500 text-sm">Download older client versions if needed</div>
                </div>
              </div>
              <a 
                href={DOWNLOADS_BASE_URL}
                className="text-accent-500 hover:text-accent-400 text-sm font-medium flex items-center space-x-1"
                target="_blank"
                rel="noopener noreferrer"
              >
                <span>View Archive</span>
                <ExternalLink className="w-4 h-4" />
              </a>
            </div>
          </div>
        </motion.div>
      </div>
    </div>
  )
}

