/* Copyright 2019-2021 Annie Lai Investments Limited
*
* Licensed under the LGPL, Version 3.0 (the "License");
* you may not use this file except in compliance with the License.
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
* You may obtain a copy of the License at the root of this project source code,
* or at:
*     https://annie lai.ai/licenses/gplv3.txt
*     https://annie lai.ai/licenses/lgplv3.txt
*/

use frame_support::{decl_module, decl_storage, dispatch::Vec};

type MessageId = u32;
type Message = Vec<u8>;

pub trait Config: frame_system::Config {}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin, system = frame_system {}
}

decl_storage! {
	trait Store for Module<T: Config> as SyloInbox {
		NextIndexes: map hasher(blake2_128_concat) T::AccountId => MessageId;
		Values get(fn values): map hasher(blake2_128_concat) T::AccountId => Vec<(MessageId, Message)>;
	}
}
