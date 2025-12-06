import React from 'react'
import { cn } from '../../utils/cn'
import { Item } from '../../types'
import * as Tooltip from '@radix-ui/react-tooltip'

interface ItemTooltipProps {
  item: Item
  children: React.ReactNode
}

export function ItemTooltip({ item, children }: ItemTooltipProps) {
  return (
    <Tooltip.Provider>
      <Tooltip.Root>
        <Tooltip.Trigger asChild>{children}</Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content
            className="z-50 max-w-xs bg-shadow-900 border border-shadow-600 rounded-lg p-3 shadow-xl animate-in fade-in-0 zoom-in-95"
            sideOffset={5}
          >
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <span className="font-semibold text-white">{item.name}</span>
              </div>
              <p className="text-sm text-shadow-400">{item.description}</p>
              <div className="space-y-1 text-sm">
                {item.attributes.attack && <div className="flex justify-between"><span className="text-shadow-500">Attack:</span><span className="text-white">{item.attributes.attack}</span></div>}
                {item.attributes.defense && <div className="flex justify-between"><span className="text-shadow-500">Defense:</span><span className="text-white">{item.attributes.defense}</span></div>}
                {item.attributes.armor && <div className="flex justify-between"><span className="text-shadow-500">Armor:</span><span className="text-white">{item.attributes.armor}</span></div>}
                {item.attributes.levelRequired && <div className="flex justify-between"><span className="text-shadow-500">Level:</span><span className="text-yellow-400">{item.attributes.levelRequired}+</span></div>}
                <div className="flex justify-between"><span className="text-shadow-500">Weight:</span><span className="text-white">{item.attributes.weight.toFixed(2)} oz</span></div>
              </div>
            </div>
            <Tooltip.Arrow className="fill-shadow-600" />
          </Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>
    </Tooltip.Provider>
  )
}

