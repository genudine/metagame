// GENERATED CODE. DO NOT EDIT MANUALLY. Run `cd hack/metagame-gen; cargo run` to generate.

pub fn alert_type(metagame_event_id: i32) -> String {
    match metagame_event_id {
        167 | 168 | 172 | 173 | 174 | 175 | 194 | 195 | 196 | 197 | 204 | 206 | 207 | 216 | 217 | 218 | 219 | 220 | 221 | 225 | 228 | 229 | 230 | 231 | 232 | 235 => "air".to_string(),
        106 | 198 | 199 | 200 | 201 | 233 | 236 | 237 | 238 | 239 | 240 | 241 => "sudden_death".to_string(),
        176 | 177 | 178 | 179 | 186 | 187 | 188 | 189 | 190 | 191 | 192 | 193 | 248 | 249 | 250 => "unstable_meltdown".to_string(),
        147 | 148 | 149 | 150 | 151 | 152 | 153 | 154 | 155 | 156 | 157 | 158 | 208 | 209 | 210 | 211 | 212 | 213 | 214 | 215 | 222 | 223 | 224 | 226 | 227 | _ => "conquest".to_string(),
    }
}