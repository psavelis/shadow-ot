'use client'

import Link from 'next/link'
import { Github, Twitter, MessageCircle, Youtube, ExternalLink } from 'lucide-react'

const footerLinks = {
  game: [
    { name: 'Download Client', href: '/download' },
    { name: 'Server Rules', href: '/rules' },
    { name: 'Getting Started', href: '/wiki/getting-started' },
    { name: 'Highscores', href: '/highscores' },
    { name: 'Server Status', href: '/status' },
  ],
  realms: [
    { name: 'Shadowveil', href: '/realms/shadowveil' },
    { name: 'Aetheria', href: '/realms/aetheria' },
    { name: 'Warbound', href: '/realms/warbound' },
    { name: 'Mythara', href: '/realms/mythara' },
    { name: 'All Realms', href: '/realms' },
  ],
  community: [
    { name: 'Forums', href: '/forum' },
    { name: 'Discord', href: 'https://discord.gg/shadowot', external: true },
    { name: 'News', href: '/news' },
    { name: 'Events', href: '/events' },
    { name: 'Guilds', href: '/guilds' },
  ],
  support: [
    { name: 'Help Center', href: '/support' },
    { name: 'Bug Reports', href: '/support/bugs' },
    { name: 'Contact Us', href: '/contact' },
    { name: 'FAQ', href: '/faq' },
    { name: 'API Docs', href: '/docs/api', external: true },
  ],
}

const socialLinks = [
  { name: 'Discord', href: 'https://discord.gg/shadowot', icon: MessageCircle },
  { name: 'Twitter', href: 'https://twitter.com/shadowot', icon: Twitter },
  { name: 'GitHub', href: 'https://github.com/psavelis/shadow-ot', icon: Github },
  { name: 'YouTube', href: 'https://youtube.com/@shadowot', icon: Youtube },
]

export function Footer() {
  return (
    <footer className="bg-shadow-950 border-t border-shadow-800">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-12 lg:py-16">
        {/* Main Footer */}
        <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-8 lg:gap-12">
          {/* Brand */}
          <div className="col-span-2 md:col-span-4 lg:col-span-1">
            <Link href="/" className="flex items-center space-x-3 mb-4">
              <div className="w-10 h-10 bg-gradient-to-br from-accent-500 to-accent-700 rounded-lg flex items-center justify-center">
                <span className="font-display font-bold text-xl text-white">S</span>
              </div>
              <span className="font-display font-bold text-xl">
                Shadow <span className="text-accent-500">OT</span>
              </span>
            </Link>
            <p className="text-shadow-400 text-sm mb-4 max-w-xs">
              The ultimate Open Tibia experience with multiple realms, blockchain integration, and unparalleled features.
            </p>
            {/* Social Links */}
            <div className="flex space-x-4">
              {socialLinks.map((link) => (
                <a
                  key={link.name}
                  href={link.href}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-shadow-400 hover:text-accent-500 transition-colors"
                  aria-label={link.name}
                >
                  <link.icon className="w-5 h-5" />
                </a>
              ))}
            </div>
          </div>

          {/* Links */}
          <div>
            <h3 className="font-semibold text-white mb-4">Game</h3>
            <ul className="space-y-2">
              {footerLinks.game.map((link) => (
                <li key={link.name}>
                  <Link
                    href={link.href}
                    className="text-shadow-400 hover:text-white text-sm transition-colors"
                  >
                    {link.name}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          <div>
            <h3 className="font-semibold text-white mb-4">Realms</h3>
            <ul className="space-y-2">
              {footerLinks.realms.map((link) => (
                <li key={link.name}>
                  <Link
                    href={link.href}
                    className="text-shadow-400 hover:text-white text-sm transition-colors"
                  >
                    {link.name}
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          <div>
            <h3 className="font-semibold text-white mb-4">Community</h3>
            <ul className="space-y-2">
              {footerLinks.community.map((link) => (
                <li key={link.name}>
                  {link.external ? (
                    <a
                      href={link.href}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-shadow-400 hover:text-white text-sm transition-colors inline-flex items-center"
                    >
                      {link.name}
                      <ExternalLink className="w-3 h-3 ml-1" />
                    </a>
                  ) : (
                    <Link
                      href={link.href}
                      className="text-shadow-400 hover:text-white text-sm transition-colors"
                    >
                      {link.name}
                    </Link>
                  )}
                </li>
              ))}
            </ul>
          </div>

          <div>
            <h3 className="font-semibold text-white mb-4">Support</h3>
            <ul className="space-y-2">
              {footerLinks.support.map((link) => (
                <li key={link.name}>
                  {link.external ? (
                    <a
                      href={link.href}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-shadow-400 hover:text-white text-sm transition-colors inline-flex items-center"
                    >
                      {link.name}
                      <ExternalLink className="w-3 h-3 ml-1" />
                    </a>
                  ) : (
                    <Link
                      href={link.href}
                      className="text-shadow-400 hover:text-white text-sm transition-colors"
                    >
                      {link.name}
                    </Link>
                  )}
                </li>
              ))}
            </ul>
          </div>
        </div>

        {/* Bottom Bar */}
        <div className="mt-12 pt-8 border-t border-shadow-800">
          <div className="flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
            <p className="text-shadow-500 text-sm">
              &copy; {new Date().getFullYear()} Shadow OT. All rights reserved.
            </p>
            <div className="flex space-x-6">
              <Link href="/privacy" className="text-shadow-500 hover:text-white text-sm transition-colors">
                Privacy Policy
              </Link>
              <Link href="/terms" className="text-shadow-500 hover:text-white text-sm transition-colors">
                Terms of Service
              </Link>
              <Link href="/cookies" className="text-shadow-500 hover:text-white text-sm transition-colors">
                Cookie Policy
              </Link>
            </div>
          </div>
        </div>
      </div>
    </footer>
  )
}
