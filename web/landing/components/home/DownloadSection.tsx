'use client'

import { motion } from 'framer-motion'
import { Download, Monitor, Apple, Gamepad2, CheckCircle } from 'lucide-react'

const clients = [
  {
    name: 'Windows Client',
    icon: Monitor,
    version: 'v12.70.1',
    size: '125 MB',
    href: '/download/windows',
    primary: true,
  },
  {
    name: 'macOS Client',
    icon: Apple,
    version: 'v12.70.1',
    size: '140 MB',
    href: '/download/macos',
    primary: false,
  },
  {
    name: 'Linux Client',
    icon: Gamepad2,
    version: 'v12.70.1',
    size: '120 MB',
    href: '/download/linux',
    primary: false,
  },
]

const features = [
  'Multi-client support (8.6 to 12.x)',
  'Built-in anti-cheat system',
  'Custom sprite support',
  'Auto-updater included',
  'Low system requirements',
  'HD graphics mode',
]

export function DownloadSection() {
  return (
    <section className="py-20 lg:py-32 bg-gradient-to-b from-shadow-900 to-shadow-950">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid lg:grid-cols-2 gap-12 items-center">
          {/* Left - Download Options */}
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.5 }}
          >
            <h2 className="font-display text-4xl lg:text-5xl font-bold mb-6">
              Ready to <span className="gradient-text">Play?</span>
            </h2>

            <p className="text-shadow-300 text-lg mb-8">
              Download our custom client and start your adventure. Compatible with all major
              operating systems and optimized for the best gaming experience.
            </p>

            {/* Download Cards */}
            <div className="space-y-4">
              {clients.map((client, index) => (
                <motion.a
                  key={client.name}
                  href={client.href}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ duration: 0.5, delay: index * 0.1 }}
                  className={`flex items-center p-4 rounded-xl border transition-all duration-300 ${
                    client.primary
                      ? 'bg-accent-500/10 border-accent-500/50 hover:bg-accent-500/20'
                      : 'bg-shadow-800/50 border-shadow-700 hover:border-shadow-500'
                  }`}
                >
                  <div className={`w-12 h-12 rounded-lg flex items-center justify-center mr-4 ${
                    client.primary ? 'bg-accent-500/20' : 'bg-shadow-700'
                  }`}>
                    <client.icon className={`w-6 h-6 ${client.primary ? 'text-accent-500' : 'text-shadow-300'}`} />
                  </div>
                  <div className="flex-grow">
                    <h3 className="font-semibold text-white">{client.name}</h3>
                    <p className="text-shadow-400 text-sm">
                      {client.version} • {client.size}
                    </p>
                  </div>
                  <Download className={`w-5 h-5 ${client.primary ? 'text-accent-500' : 'text-shadow-400'}`} />
                </motion.a>
              ))}
            </div>

            {/* Alternative */}
            <p className="mt-6 text-shadow-500 text-sm">
              Or use any OT client compatible with protocol 12.70.{' '}
              <a href="/docs/client-setup" className="text-accent-500 hover:text-accent-400">
                Learn more
              </a>
            </p>
          </motion.div>

          {/* Right - Features */}
          <motion.div
            initial={{ opacity: 0, x: 50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.5 }}
          >
            <div className="bg-shadow-900/50 rounded-2xl border border-shadow-700 p-8">
              <h3 className="font-display text-2xl font-bold mb-6 text-white">
                Client Features
              </h3>

              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                {features.map((feature, index) => (
                  <motion.div
                    key={feature}
                    initial={{ opacity: 0, y: 10 }}
                    whileInView={{ opacity: 1, y: 0 }}
                    viewport={{ once: true }}
                    transition={{ duration: 0.3, delay: 0.3 + index * 0.05 }}
                    className="flex items-center space-x-3"
                  >
                    <CheckCircle className="w-5 h-5 text-green-500 flex-shrink-0" />
                    <span className="text-shadow-300">{feature}</span>
                  </motion.div>
                ))}
              </div>

              {/* System Requirements */}
              <div className="mt-8 pt-6 border-t border-shadow-700">
                <h4 className="font-semibold text-white mb-4">Minimum Requirements</h4>
                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <span className="text-shadow-500">OS:</span>
                    <span className="text-shadow-300 ml-2">Windows 10 / macOS 11 / Ubuntu 20</span>
                  </div>
                  <div>
                    <span className="text-shadow-500">RAM:</span>
                    <span className="text-shadow-300 ml-2">4 GB</span>
                  </div>
                  <div>
                    <span className="text-shadow-500">Storage:</span>
                    <span className="text-shadow-300 ml-2">500 MB</span>
                  </div>
                  <div>
                    <span className="text-shadow-500">Network:</span>
                    <span className="text-shadow-300 ml-2">Broadband</span>
                  </div>
                </div>
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  )
}
